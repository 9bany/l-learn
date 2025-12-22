class Node:
    def __init__(self, val: int):
        self.val = val
        self.next = None

class Deque:
    
    def __init__(self):
        self.head = None
        self.tail = None

    def isEmpty(self) -> bool:
        return self.head == None and self.tail == None

    def append(self, value: int) -> None:
        new = Node(val=value)
        if self.head is None:
            self.head = new
            self.tail = new
            return
        self.tail.next = new
        self.tail = new

    def appendleft(self, value: int) -> None:
        new = Node(val=value)
        if self.head is None:
            self.head = new
            self.tail = new
            return
        new.next = self.head
        self.head = new

    def pop(self) -> int:
        if self.head is None:
            return -1
        val = self.head.val
        if self.head == self.tail:
            self.head = None
            self.head = None
            return val
        self.head = self.head.next
        return val

    def popleft(self) -> int:
        if self.head is None:
            return -1
        if self.head == self.tail:
            val = self.head.val
            self.head = None
            self.head = None
            return val
        val = self.tail.val
        x
        cur = self.cur
        pre = None
        while(cur.next is not None):
            pre = cur
            cur = cur.next
        if pre is not None:
            pre.next = None
        else:
            self.cur = None
        return cur.val