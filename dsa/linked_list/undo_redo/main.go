package main

import "fmt"

type Action struct {
	command string
	prev    *Action
	next    *Action
}

type UndoRedo struct {
	head     *Action
	curr     *Action
	snapshot *Action
	limit    int
	count    int
}

func NewUndoRedo(limit int) *UndoRedo {
	return &UndoRedo{
		limit: limit,
	}
}

func (u *UndoRedo) Do(command string) {
	newAction := &Action{
		command: command,
		prev:    u.curr,
		next:    nil,
	}
	if u.curr == nil {
		u.curr = newAction
		u.head = newAction
	} else {
		u.curr.next = newAction
		u.curr = newAction
	}

	u.count++
	if u.count > u.limit {
		u.head = u.head.next
		u.head.prev = nil
		u.count--
	}

}

func (u *UndoRedo) Redo(n int) {
	for i := 0; i < n && u.curr != nil && u.curr.next != nil; i++ {
		u.curr = u.curr.next
	}
}

func (u *UndoRedo) Undo(n int) {
	for i := 0; i < n && u.curr != nil && u.curr.prev != nil; i++ {
		u.curr = u.curr.prev
	}
}

func (ur *UndoRedo) PrintCurrent() {
	if ur.curr != nil {
		fmt.Println("Current Action:", ur.curr.command)
	} else {
		fmt.Println("No Action Available")
	}
}

func (ur *UndoRedo) Snapshot() *Action {
	return ur.curr
}

func (ur *UndoRedo) LoadSnapshot(snapshot *Action) {
	if snapshot != nil {
		ur.curr = snapshot
	}
}

func (u *UndoRedo) ListHistory() {
	fmt.Println("History of Actions:")
	action := u.head
	for action != nil {
		fmt.Println("-", action.command)
		action = action.next
	}
}

func main() {
	ur := NewUndoRedo(5) // Giới hạn lưu trữ 5 hành động gần nhất

	ur.Do("Open File")
	ur.Do("Edit Line 1")
	ur.Do("Edit Line 2")
	ur.Do("Delete Word")
	ur.Do("Save File")
	ur.Do("Close File") // Lúc này "Open File" sẽ bị xóa khỏi lịch sử

	fmt.Println("==== Before Undo ====")
	ur.PrintCurrent()

	// Undo 2 bước
	ur.Undo(2)
	fmt.Println("==== After Undo (2 steps) ====")
	ur.PrintCurrent()

	// Lưu snapshot
	snapshot := ur.Snapshot()

	// Redo 1 bước
	ur.Redo(1)
	fmt.Println("==== After Redo (1 step) ====")
	ur.PrintCurrent()

	// Quay lại snapshot
	ur.LoadSnapshot(snapshot)
	fmt.Println("==== After Loading Snapshot ====")
	ur.PrintCurrent()

	// In lịch sử
	ur.ListHistory()
}
