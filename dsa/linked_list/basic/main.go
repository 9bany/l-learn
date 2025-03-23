package main

import (
	"fmt"
	"strings"
)

// Node struct
type Node struct {
	value int
	next  *Node
}

// LinkedList struct
type LinkedList struct {
	head *Node
}

func (ll *LinkedList) Search(value int) bool {
	n := ll.head
	for n != nil {
		if n.value == value {
			return true
		}
		n = n.next
	}
	return false
}

func (ll *LinkedList) Size() (result int) {
	n := ll.head
	for n != nil {
		result++
		n = n.next
	}
	return result
}

// Insert: Thêm node vào cuối danh sách
func (ll *LinkedList) Insert(value int) *Node {
	newNode := &Node{
		value: value,
	}
	n := ll.head

	if n == nil {
		ll.head = newNode
		return newNode
	}

	for n.next != nil {
		n = n.next
	}
	n.next = newNode
	return newNode
}

// Delete: Xóa node có giá trị value
func (ll *LinkedList) Delete(value int) {
	if ll.head != nil && ll.head.value == value {
		ll.head = ll.head.next
		return
	}
	n := ll.head
	for n != nil {
		if n.next != nil {
			if n.next.value == value {
				n.next = n.next.next
				break
			}
		}
		n = n.next
	}
}

// Print: In danh sách
func (ll *LinkedList) Print() {
	var result []string = []string{}
	n := ll.head
	for n != nil {
		result = append(result, fmt.Sprint(n.value))
		n = n.next
	}
	fmt.Println(fmt.Sprintf("Output: %s", strings.Join(result, " -> ")))
}

func (ll *LinkedList) Reverse() {
	var prev, curr, next *Node
	curr = ll.head
	for curr != nil {
		next = curr.next
		curr.next = prev
		prev = curr
		curr = next
	}
	ll.head = prev
}

func (ll *LinkedList) DetectCycle() bool {
	l := make(map[*Node]bool)
	n := ll.head
	for n != nil {
		if _, ok := l[n]; ok {
			return true
		} else {
			l[n] = true
		}
		n = n.next
	}
	return false
}

func (ll *LinkedList) FloydDetectCycle() bool {
	fast := ll.head
	slow := ll.head

	for fast != nil && fast.next != nil {
		slow = slow.next
		fast = fast.next.next

		if fast == slow {
			return true
		}
	}
	return false
}

func (ll *LinkedList) FindCycleStart() *Node {
	fast := ll.head
	slow := ll.head

	for fast != nil && fast.next != nil {
		slow = slow.next
		fast = fast.next.next

		if fast == slow {
			slow = ll.head
			for fast != slow {
				slow = slow.next
				fast = fast.next
			}
			return fast
		}
	}
	return nil
}

func (ll *LinkedList) FindMiddle() *Node {
	fast := ll.head
	slow := ll.head

	for fast != nil && fast.next != nil {
		slow = slow.next
		fast = fast.next.next
	}
	return slow
}

func (ll *LinkedList) RemoveNthFromEnd(n int) {
	fast := ll.head
	slow := ll.head

	i := 1
	for i <= n {
		fast = fast.next
		i++
	}

	if fast == nil {
		ll.head = ll.head.next
		return
	}

	for fast.next != nil {
		slow = slow.next
		fast = fast.next
	}
	slow.next = slow.next.next
}

func (ll *LinkedList) ReorderList() {
	fast := ll.head
	slow := ll.head

	for fast != nil && fast.next != nil {
		slow = slow.next
		fast = fast.next.next
	}

	secondHaft := &LinkedList{
		head: slow.next,
	}
	slow.next = nil
	secondHaft.Reverse()

	p1 := ll.head
	p2 := secondHaft.head

	for p2 != nil {

		tmp1, tmp2 := p1.next, p2.next

		p1.next = p2
		p2.next = tmp1

		p1 = tmp1
		p2 = tmp2
	}

}

func (ll *LinkedList) RemoveDuplicatesOrdered() {
	curr := ll.head
	for curr != nil && curr.next != nil {
		if curr.value == curr.next.value {
			curr.next = curr.next.next
		}
		curr = curr.next
	}
}

func (ll *LinkedList) RemoveDuplicates() {
	store := make(map[int]bool)
	curr := ll.head
	prev := ll.head
	for curr != nil {
		if _, ok := store[curr.value]; ok {
			prev.next = curr.next
		} else {
			store[curr.value] = true
			prev = curr
		}
		curr = curr.next
	}
}

func (ll *LinkedList) ReverseKGroup(k int) *LinkedList {
	i := 0
	curr := ll.head
	for curr != nil && i < k {
		curr = curr.next
		i++
	}
	if i == k {
		curr = ll.head
	} else {
		return ll
	}

	i = 0
	var prev *Node
	var next *Node

	for i < k && curr != nil {
		next = curr.next
		curr.next = prev
		prev = curr
		curr = next
		i++
	}
	f := &LinkedList{head: curr}
	ll.head.next = f.ReverseKGroup(k).head
	return &LinkedList{head: prev}
}

func MergeTwoSortedLists(l1, l2 *LinkedList) *LinkedList {
	ll1 := l1.head
	ll2 := l2.head

	new := &LinkedList{}

	for ll1 != nil || ll2 != nil {
		if ll1.value <= ll2.value {
			new.Insert(ll1.value)
			ll1 = ll1.next
		} else {
			new.Insert(ll2.value)
			ll2 = ll2.next
		}
	}

	for ll1 != nil {
		new.Insert(ll1.value)
		ll1 = ll1.next
	}

	for ll2 != nil {
		new.Insert(ll2.value)
		ll2 = ll2.next
	}

	return new
}

func FindIntersection(l1, l2 *LinkedList) *Node {
	p1 := l1.head
	p2 := l2.head

	for p1 != p2 {
		if p1 == nil {
			p1 = l2.head
		} else {
			p1 = p1.next
		}

		if p2 == nil {
			p2 = l1.head
		} else {
			p2 = p2.next
		}
	}
	return p1
}

func main() {
	ll := &LinkedList{}
	for _, v := range []int{1, 2, 3, 4, 5, 6, 7, 8} {
		ll.Insert(v)
	}

	fmt.Println("Original List:")
	ll.Print()

	ll = ll.ReverseKGroup(3)
	fmt.Println("After Reversing in K-Group:")
	ll.Print()
}
