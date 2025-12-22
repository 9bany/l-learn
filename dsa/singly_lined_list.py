class Node:
    def __init__(self, value: int, next: Node = None):
        self.next = next
        self.value = value

class LinkedList:
    
    def __init__(self):
        self.head = None

    def get(self, index: int) -> int:
        cur = self.head
        i = 0
        while(cur is not None):
            if index == i:
                return cur.value
            i = i + 1
            cur = cur.next
        return -1
        

    def insertHead(self, val: int) -> None:
        new = Node(value=val)
        if self.head is None:
            self.head = new
            return
        new.next = self.head
        self.head = new
        

    def insertTail(self, val: int) -> None:
        new = Node(value=val)
        if self.head is None:
            self.head = new
        else:
            cur = self.head
            while(cur.next is not None):
                cur = cur.next
            cur.next = Node(value=val)

    def remove(self, index: int) -> bool:
        if index == 0 and self.head is not None:
            self.head = self.head.next
            return True
        
        previous = None
        cur = self.head
        
        i = 0
        while(cur is not None):
            if index == i:
                previous.next = cur.next
                return True
            previous = cur
            cur = cur.next
            i = i + 1
        return False

    def getValues(self) -> List[int]:
        result: List[int] = []
        cur = self.head
        while(cur is not None):
            result.append(cur.value)
            cur = cur.next
        return result
        
