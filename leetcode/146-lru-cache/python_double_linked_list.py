from typing import Optional, Dict


class Entry:

    def __init__(
        self,
        key: int,
        value: int,
    ):
        self.key = key
        self.value = value
        self.next: Optional[Entry] = None
        self.prev: Optional[Entry] = None

    def __str__(self):
        return "{}={}".format(self.key, self.value)

    def __repr__(self):
        return self.__str__()


class LRUCache:

    def __init__(self, capacity: int):
        # head contains the least recently used
        self.size = 0
        # head of the doubly linked list. head.next will always point to the least recently used
        self.head = Entry(-1, -1)
        # tail is what we will use to append (tail.prev is the last node) new entries when inserting or accessing a key.
        self.tail = Entry(-1, -1)
        self.head.next = self.tail
        self.tail.prev = self.head
        self.items: Dict[int, Entry] = {}
        self.capacity = capacity


    def _remove(self, entry: Entry):
        """Run time is O(1) as pointer changing is constant"""
        entry.prev.next = entry.next
        entry.next.prev = entry.prev

    
    def _append(self, entry: Entry):
        """Run time is O(1) as pointer changing is constant"""
        prev = self.tail.prev
        prev.next = entry
        entry.prev = prev
        entry.next = self.tail
        self.tail.prev = entry

    def get(self, key: int) -> int:
        """Run time is O(1) as remove and append are constant"""
        e = self.items.get(key)
        if not e:
            return -1

        # remove and append to the tail now (make it "more recent")
        self._remove(e)
        self._append(e)

        return e.value

    def put(self, key: int, value: int) -> None:
        """Run time is O(1) since remove and append are cosntant, as well as dict.pop (ish)"""
        e = Entry(key, value)

        # need to invalidate cache if it exists
        if key in self.items:
            invalidated = self.items.pop(e.key)
            self._remove(invalidated)
            self.size -= 1

        self._append(e)
        self.items[key] = e

        # todo, handle edge case when we are at capacity
        if self.size >= self.capacity:
            candidate = self.head.next
            self._remove(candidate)
            self.items.pop(candidate.key)
        else:
            self.size += 1


# Your LRUCache object will be instantiated and called as such:
# obj = LRUCache(capacity)
# param_1 = obj.get(key)
# obj.put(key,value)

def test_basic():
    cache =  LRUCache(2)
    cache.put(1, 1)
    cache.put(2, 2)
    cache.put(3,3)

    assert cache.get(1) == -1
    assert cache.get(2) == 2
    cache.put(4, 4)
    assert cache.get(3) == -1
    assert cache.get(4) == 4


def test_can_update():
    cache =  LRUCache(2)
    cache.put(2, 1)
    cache.put(2, 2)

    assert cache.size == 1
    assert cache.get(2) == 2
    cache.put(1, 1)
    cache.put(4, 1)
    assert cache.get(2) == -1
