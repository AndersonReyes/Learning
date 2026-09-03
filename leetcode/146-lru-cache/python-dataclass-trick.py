import dataclasses
import heapq
import time


@dataclasses.dataclass
class Entry:
    """A cached value and the time it was most recently accessed.

    Entries are ordered by timestamp so ``heapq`` places the least recently
    used entry at the root of its min-heap.
    """

    timestamp: float
    key: int
    value: int

    def __le__(self, other: "Entry") -> bool:
        """Return whether this entry is no newer than another entry."""
        return self.timestamp <= other.timestamp

    def __lt__(self, other: "Entry") -> bool:
        """Order entries by timestamp for use in the min-heap."""
        return self.timestamp < other.timestamp


class LRUCache:
    """A least-recently-used cache backed by a heap and a hash map.

    The heap orders entries by their last-access timestamp, while ``items``
    provides constant-time lookup by key. Both collections contain references
    to the same Entry objects.

    Updating an entry's timestamp mutates its heap priority. Because ``heapq``
    cannot efficiently update arbitrary priorities, ``get`` rebuilds the heap
    after every successful lookup.

    Complexity:
        get:
            Average O(1) lookup followed by O(n) heap reconstruction.
            Overall: O(n).

        put:
            O(log n) for heap insertion or replacement.
            Average O(1) for dictionary operations.
            Overall: O(log n).

        space:
            O(capacity).

    Note:
        This implementation assumes ``put`` is called only for keys that are
        not already present. Inserting an existing key creates a stale heap
        entry and can later remove the wrong dictionary mapping.
    """

    def __init__(self, capacity: int):
        """Initialize a cache with the given maximum number of entries.

        Args:
            capacity: Maximum number of entries retained by the cache.
        """
        self.pq: list[Entry] = []
        self.items: dict[int, Entry] = {}
        self.capacity = capacity

    def get(self, key: int) -> int:
        """Return a cached value and mark its entry as recently used.

        Args:
            key: Key to retrieve.

        Returns:
            The cached value, or -1 when the key is absent.

        Complexity:
            O(n), because changing the timestamp requires rebuilding the heap.
        """
        item = self.items.get(key)
        if item is None:
            return -1

        # The heap and dictionary share this Entry object, so changing its
        # timestamp also changes the object stored in the heap.
        item.timestamp = time.time()

        # Restore heap ordering after mutating an entry's priority.
        heapq.heapify(self.pq)
        return item.value

    def put(self, key: int, value: int) -> None:
        """Insert a value and evict the least recently used entry if full.

        Args:
            key: Key associated with the value.
            value: Value to cache.

        Complexity:
            O(log n) for ``heappush`` or ``heappushpop``.
        """
        entry = Entry(time.time(), key, value)
        print("adding:", entry)

        if len(self.pq) >= self.capacity:
            # Since this is a min-heap ordered by timestamp, the returned
            # entry is the least recently used one.
            replaced = heapq.heappushpop(self.pq, entry)
            print("replace:", replaced)
            del self.items[replaced.key]
        else:
            heapq.heappush(self.pq, entry)

        self.items[key] = entry
