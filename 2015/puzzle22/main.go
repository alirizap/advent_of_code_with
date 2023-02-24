// Original solution https://github.com/arnokamphuis/AdventOfCode/blob/master/2015/day22/src/day22_15.rs
package main

import "fmt"

type State struct {
	playerHP int
	bossHP   int
	mana     int
}

func doTurn(state State, activeSpells [][]int, playerTurn bool, manaUsed int, least *int, part int) bool {
	spells := [][]int{
		{53, 4, 0, 0, 0, 0, 0},
		{73, 2, 2, 0, 0, 0, 1},
		{113, 0, 0, 7, 0, 6, 2},
		{173, 3, 0, 0, 0, 6, 3},
		{229, 0, 0, 0, 101, 5, 4},
	}
	bossDamage := 8
	playerArmour := 0

	newState := state

	if part == 2 && playerTurn {
		newState.playerHP -= 1
		if newState.playerHP <= 0 {
			return false
		}
	}

	newActiveSpells := [][]int{}
	for _, activeSpell := range activeSpells {
		if activeSpell[5] >= 0 {
			newState.bossHP -= activeSpell[1]
			newState.playerHP += activeSpell[2]
			playerArmour += activeSpell[3]
			newState.mana += activeSpell[4]
		}

		newActiveSpell := make([]int, len(activeSpell))
		copy(newActiveSpell, activeSpell)
		newActiveSpell[5] -= 1
		if newActiveSpell[5] > 0 {
			newActiveSpells = append(newActiveSpells, newActiveSpell)
		}
	}
	if newState.bossHP <= 0 {
		if manaUsed < *least || *least == -1 {
			*least = manaUsed
		}
		return true
	}

	if manaUsed >= *least && *least != -1 {
		return false
	}

	if playerTurn {
		for _, spell := range spells {
			active := false
			for _, t := range newActiveSpells {
				if t[6] == spell[6] {
					active = true
					break
				}
			}
			manaCost := spell[0]
			if manaCost <= newState.mana && !active {
				nas := make([][]int, len(newActiveSpells))
				copy(nas, newActiveSpells)
				nas = append(nas, spell)
				nextState := newState
				nextState.mana -= manaCost
				doTurn(nextState, nas, false, manaUsed+manaCost, least, part)
			}
		}
	} else {
		if playerArmour-bossDamage < 0 {
			newState.playerHP += playerArmour - bossDamage
		} else {
			newState.playerHP -= 1
		}
		if newState.playerHP > 0 {
			return doTurn(newState, newActiveSpells, true, manaUsed, least, part)
		}
	}
	return false
}

func run() {
	least := -1
	state := State{bossHP: 55, playerHP: 50, mana: 500}
	doTurn(state, [][]int{}, true, 0, &least, 1)
	fmt.Println("Part One: ", least)

	least = -1
	state = State{bossHP: 55, playerHP: 50, mana: 500}
	doTurn(state, [][]int{}, true, 0, &least, 2)
	fmt.Println("Part Two: ", least)
}

func main() {
	run()
}
