package main

import "fmt"

type Action struct {
	command string
	prev    *Action
	next    *Action
}

type UndoRedo struct {
	curr *Action
}

func (u *UndoRedo) Do(command string) {
	newAction := &Action{
		command: command,
		prev:    u.curr,
		next:    nil,
	}
	if u.curr == nil {
		u.curr = newAction
	} else {
		u.curr.next = newAction
		u.curr = newAction
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

func main() {
	ur := &UndoRedo{}

	// Thêm các hành động
	ur.Do("Open File")
	ur.Do("Edit Line 1")
	ur.Do("Edit Line 2")
	ur.Do("Delete Word")
	ur.Do("Save File")

	fmt.Println("==== Before Undo ====")
	ur.PrintCurrent()

	// Undo 2 lần
	ur.Undo(2)
	fmt.Println("==== After Undo (2 steps) ====")
	ur.PrintCurrent()

	// Redo 1 lần
	ur.Redo(1)
	fmt.Println("==== After Redo (1 step) ====")
	ur.PrintCurrent()
}
