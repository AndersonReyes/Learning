System design: caching 

Where to cache
1. You can have a separate shared cluster to cache (redis, memcache etc). One cached request can be reused across all the application servers. Cache cluster can scale independently. Slower than in app memory cluster of course due to network request. Cheaper since we are sharing one cluster.
2. In ram caching on each application servers. Requires each server to have enough ram. More costly since each application server large ram. Cache data is duplicated across each server if we are load balancing. Fastest response since ram is much faster than network request. 
3. You could in theory have a combination of the above. Small in memory cache for hot keys and then proceed to shared cache for everything else. This allows handling hot keys well. 

Eviction policies
1. Lest recently used: maintains a data structure of keys and access time order by access time. Can use a min heap with the last access times. Updates require popping and adding again but these operations are about log n. Removing the oldest is constant time. Can also use pointer in the value of the key that is also reused in a double linked list. Removing is just updating the forward and backward pointers and then reading to the head or tail depending on if you want the least at the head or tail.
2. Least frequently used uses a counter for each key. It drops the one with the lowest access count. Again can use a heap to order and keep the lowest count key constant access.
3. Time to live uses an interval  to automatically remove the key after interval passes. You do not have to delete the records right away but the ttl determines validity when accessing.

One thing to note is that even with eviction policy data can still go stale. Db can be updated externally which makes the cache data stale. Using a tll helps with this to eventually refresh the data from the database. It makes the system eventually consistent. 

Cache Write policies 
1. Write to cache first then to database (sequential or parallel). Sequentially is slower but guarantees the db is written before returning. Asynchronous is faster but if an error occurs (can also happen in sequential) after the cache is updated the data now lives in the cache but not in the database so you create an inconsistent state. You could fix this by partially recovering from the error to invalidate the cache entry before propagating the error.
2. write to the database first then the cache. If an error occurs after db is written, no harm is caused as the next fetch will reach the database first after cache miss and populate the data in the cache.

Hot keys 
* hot keys can be handled by replicating  the hot key data across multiple servers. This allows us to load balance hotkeys between multiple servers. 
* 

Scaling
1. Adding multiple nodes allows load balancing and handle Hot keys. This can be expensive for non hotkeys. 
2. Initial deployment can suffer from large cache misses and all requests retrieving the objects and writing to cache overloading the server (thundering herd problem). This can be avoided by pre warning up the cache with  data (expected to be frequently requested, popular keys)
3. Hot key problem: With the data across multiple nodes you can use salting to distribute multiple queries for the same key across multiple nodes. Can also use small local server only cache for hot keys

Data portioning
1. Hash the keys to nodes consistently. Can’t use key mod N because N can change if there are failures. Need to use consistent hashing which allows for node failures. The keys are hashed in a way that when one node fails. Only that one node needs data redistribution. 
2. To add redundancy we can replicate the data across multiple nodes. If a node goes down, there are other copies to rebuild the new node from

Concurrency 
1. Use thread pools to avoid unbounded penalty of new threads or connections
2. Can use locks to avoid race conditions. Locks do increase latency specially if there is a lot of contention between the same keys  
3. Add read replicas to the cluster would make reads super fast while keep the write traffic separate from the reads
4. 
