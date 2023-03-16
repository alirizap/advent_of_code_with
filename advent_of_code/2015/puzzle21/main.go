package main

import (
	"fmt"
	"math/bits"
	"os"
	"strconv"
	"strings"
)

const MaxUint = ^uint(0) 
const MaxInt = int(MaxUint >> 1) 
const MinInt = -MaxInt - 1

var shopMap = map[string]*Item{
	"Dagger":     newItem(8, 4, 0),
	"Shortsword": newItem(10, 5, 0),
	"Warhammer":  newItem(25, 6, 0),
	"Longsword":  newItem(40, 7, 0),
	"Greataxe":   newItem(74, 8, 0),
	"Leather":    newItem(13, 0, 1),
	"Chainmail":  newItem(31, 0, 2),
	"Splintmail": newItem(53, 0, 3),
	"Bandedmail": newItem(75, 0, 4),
	"Platemail":  newItem(102, 0, 5),
	"Damage+1":   newItem(25, 1, 0),
	"Damage+2":   newItem(50, 2, 0),
	"Damage+3":   newItem(100, 3, 0),
	"Defense+1":  newItem(20, 0, 1),
	"Defense+2":  newItem(40, 0, 2),
	"Defense+3":  newItem(80, 0, 3),
}

func combinations(set []string, n int) (subsets [][]string) {
	length := uint(len(set))
	for subsetBits := 1; subsetBits < (1 << length); subsetBits++ {
		if n > 0 && bits.OnesCount(uint(subsetBits)) != n {
			continue
		}
		var subset []string
		for object := uint(0); object < length; object++ {
			if (subsetBits>>object)&1 == 1 {
				subset = append(subset, set[object])

			}
		}
		subsets = append(subsets, subset)
	}
	return subsets
}

func parseString(content string) []int {
	data := strings.Split(content, "\r\n")
	values := make([]int, 0, 3)
	for _, d := range data {
		_, after, _ := strings.Cut(d, ":")
		value, err := strconv.Atoi(strings.TrimSpace(after))
		if err != nil {
			panic(err)
		}
		values = append(values, value)
	}
	return values
}

type Item struct {
	cost, damage, armor int
}

func newItem(cost, damage, armor int) *Item {
	return &Item{cost, damage, armor}
}

type Character struct {
	hit_point int
	damage    int
	armor     int
}

func (c *Character) addItem(item string, cost *int) {
	c.armor += shopMap[item].armor
	c.damage += shopMap[item].damage
	*cost += shopMap[item].cost
}

func newCharacter(hit_point, damage, armor int) *Character {
	return &Character{hit_point, damage, armor}
}


func fight(player, boss *Character) bool {
	for {
		p_damage := player.damage - boss.armor
		boss.hit_point -= p_damage
		if boss.hit_point <= 0 {
			return true
		}

		b_damage := boss.damage - player.armor
		player.hit_point -= b_damage
		if player.hit_point <= 0 {
			return false
		}
	}
}

func run(content string) {

	var weapons = []string{"Dagger", "Shortsword", "Warhammer", "Longsword", "Greataxe"}
	var armor = []string{"Leather", "Chainmail", "Splintmail", "Bandedmail", "Platemail"}
	var rings = []string{"Damage+1", "Damage+2", "Damage+3", "Defense+1", "Defense+2", "Defense+3"}

	ring_combs := combinations(rings, 1)
	ring_combs = append(ring_combs, combinations(rings, 2)...)
	bossValues := parseString(content)
	leastAmount := MaxInt
	mostAmount := MinInt
	for _, w := range weapons {
		for i := -1; i < len(armor); i++ {
			for _, r := range ring_combs {
				cost := 0
				boss := newCharacter(bossValues[0], bossValues[1], bossValues[2])
				player := newCharacter(100, 0, 0)
				if i != -1 {
					player.addItem(armor[i], &cost)
				}
				player.addItem(w, &cost)
				player.addItem(r[0], &cost)
				if len(r) > 1 {
					player.addItem(r[1], &cost)
				}

				win := fight(player, boss);
				if win && cost < leastAmount {
					leastAmount = cost 
				}

				if !win && cost > mostAmount {
					mostAmount = cost 
				}
			}
		}
	}
	fmt.Println("Part One: ", leastAmount)
	fmt.Println("Part Two: ", mostAmount)
}

func main() {
	content, err := os.ReadFile("20")
	if err != nil {
		panic(err)
	}
	run(string(content))
}
