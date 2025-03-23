package main

import (
	"fmt"
	"log"
	"strings"
)

type Node struct {
	key, val   int
	prev, next *Node
}

type LRUCache struct {
	head, tail *Node
	capacity   int
	cache      map[int]*Node
}

func Constructor(capacity int) LRUCache {
	return LRUCache{
		capacity: capacity,
		cache:    make(map[int]*Node),
	}
}

func (lru *LRUCache) Print() {
	result := []string{}
	n := lru.head
	for n != nil {
		result = append(result, fmt.Sprintf("%d", n.key))
		n = n.next
	}
	fmt.Printf("Chain from head: %s\n", strings.Join(result, " -> "))
	fmt.Printf("Tail: key - %d and val - %d \n", lru.tail.key, lru.tail.val)
}

func (lru *LRUCache) Get(key int) int {
	node, ok := lru.cache[key]
	if !ok {
		return -1
	}
	lru.remove(node)
	lru.addToHead(node)
	return node.val
}

func (lru *LRUCache) Put(key int, value int) {
	node, ok := lru.cache[key]
	if !ok {
		node = &Node{
			key: key,
			val: value,
		}
		lru.cache[key] = node
	} else {
		node.val = value
		lru.remove(node)
	}

	lru.addToHead(node)

	if len(lru.cache) > lru.capacity {
		last := lru.tail
		lru.remove(last)
		delete(lru.cache, last.key)
	}
}

func (lru *LRUCache) remove(node *Node) {
	if node == lru.head {
		lru.head = node.prev
	}

	if node == lru.tail {
		lru.tail = node.next
	}

	if node.next != nil {
		node.next.prev = node.prev
	}

	if node.prev != nil {
		node.prev.next = node.next
	}
}

func (lru *LRUCache) addToHead(node *Node) {
	node.next = nil

	if lru.head != nil {
		lru.head.next = node
		node.prev = lru.head
	}

	lru.head = node

	if lru.tail == nil {
		lru.tail = node
	}
}

func main() {
	lru := Constructor(4)
	lru.Put(1, 1)
	lru.Put(2, 2)
	lru.Put(3, 3)
	lru.Put(4, 4)
	log.Println(lru.Get(3))
	lru.Put(5, 5)
	lru.Put(6, 6)
	lru.Print()
}
