# 12. DNS Protocol + Minimal DNS Resolver Over UDP

## Domain name encoding (RFC 1035 §4.1.2, §2.3.4)

DNS encodes domain names as a sequence of **length-prefixed labels**,
terminated by a zero-length label:

```
"example.com" ->  7 e x a m p l e  3 c o m  0
                  ^len            ^len      ^terminator
```

- Each label is prefixed by a single length byte (0-63 — the top two bits
  must be `00`, so the maximum label length is 63 bytes).
- The root name `""` encodes as a single zero byte.
- An empty label (e.g. from `"a..b"`) or a label over 63 bytes is invalid.

## Message compression (RFC 1035 §4.1.4)

To avoid repeating the same domain name in every answer record, DNS
messages use **compression pointers**: a length byte whose top two bits are
both `1` (`0xC0`) is not a length — it's the first byte of a 2-byte pointer.
The remaining 14 bits (6 from this byte, 8 from the next) are an offset from
the start of the message where the rest of the name continues:

```
byte 0: 11OOOOOO   <- top 2 bits = 11 marks a pointer
byte 1: OOOOOOOO   <- combined 14-bit offset O
```

A name can mix literal labels and a trailing pointer (`www` + pointer to
`example.com` already seen earlier in the message). When decoding, a pointer
may itself point at a name that contains another pointer — real
implementations cap the number of jumps followed to avoid an infinite loop
on a malicious or corrupt message.

The byte position returned by a name-decoding routine for "what comes after
this name" is subtle once pointers are involved: it's the offset
**immediately after the pointer's two bytes** in the *current* location,
not anything inside the pointed-to data (which lives elsewhere in the
message and is unrelated to the current record's layout).

## Message header (RFC 1035 §4.1.1)

A DNS message starts with a fixed 12-byte header:

| Bytes | Field | Meaning |
|---|---|---|
| 0-1 | ID | Query ID, echoed back in the response |
| 2-3 | flags | QR, Opcode, AA, TC, **RD**, RA, Z, RCODE |
| 4-5 | QDCOUNT | number of entries in the question section |
| 6-7 | ANCOUNT | number of entries in the answer section |
| 8-9 | NSCOUNT | number of entries in the authority section |
| 10-11 | ARCOUNT | number of entries in the additional section |

Within the 16-bit flags field, the low 4 bits are **RCODE** (0 = no error,
3 = NXDOMAIN — name does not exist). The **RD** (recursion desired) bit is
bit 8 (i.e. `0x0100`) — a resolver sets it to ask the server to fully
resolve the name rather than just return what it has cached.

## Question section (RFC 1035 §4.1.2)

Following the header, each question is:

```
QNAME  (encoded domain name, as above)
QTYPE  (2 bytes, e.g. 1 = A record)
QCLASS (2 bytes, 1 = IN, "Internet")
```

## Resource record format (RFC 1035 §4.1.3)

Each answer/authority/additional record is:

```
NAME     (encoded domain name, often a compression pointer)
TYPE     (2 bytes)
CLASS    (2 bytes)
TTL      (4 bytes, seconds)
RDLENGTH (2 bytes)
RDATA    (RDLENGTH bytes, format depends on TYPE)
```

For a **TYPE A** record (`TypeA = 1`, RFC 1035 §3.2.2), `RDLENGTH` is 4 and
`RDATA` is the 4-byte IPv4 address.

---

## Networking: DNS over UDP, port 53

[RFC 1035](https://www.rfc-editor.org/rfc/rfc1035) specifies DNS messages
identically for UDP and TCP; topic 9's notes mentioned the TCP framing
(2-byte length prefix, §4.2.2). Over UDP — the common case for individual
A-record lookups — a client:

1. Builds a query message (header with a fresh, ideally random, `ID` and
   `RD` set, plus one question) and sends it as a single UDP datagram to
   port 53.
2. Reads a single UDP datagram back and parses it as a response.
3. Checks that the response `ID` matches the query `ID` — this is the
   primary defense against a third party (off-path attacker) injecting a
   forged response, since UDP has no connection state to verify against.
   [RFC 5452](https://www.rfc-editor.org/rfc/rfc5452) recommends combining
   this with a randomized **source port** and randomized query `ID` for
   stronger spoofing resistance — real resolvers do both; this exercise's
   `Resolve` randomizes the `ID` (`math/rand`) and relies on
   `net.DialUDP` for an ephemeral source port.
4. Checks `RCODE` — a non-zero value (e.g. 3 = NXDOMAIN) means the query
   failed even though a response arrived.

Because UDP can silently drop the response, `Resolve` takes a `timeout` and
uses `SetReadDeadline` (the same pattern as topic 7's connection deadlines)
to bound how long it waits.

## Further Reading

- [RFC 1035 (Domain Names — Implementation and Specification)](https://www.rfc-editor.org/rfc/rfc1035)
- [RFC 1035 §4.1 (Message format)](https://www.rfc-editor.org/rfc/rfc1035#section-4.1)
- [RFC 1035 §3.2.2 (TYPE values)](https://www.rfc-editor.org/rfc/rfc1035#section-3.2.2)
- [RFC 5452 (Measures for DNS Spoofing Resistance)](https://www.rfc-editor.org/rfc/rfc5452)
- [`encoding/binary`](https://pkg.go.dev/encoding/binary)
- [`net.DialUDP`](https://pkg.go.dev/net#DialUDP), [`net.IP`](https://pkg.go.dev/net#IP)
