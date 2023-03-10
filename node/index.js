#!/usr/bin/env node

const a = "abc"
let b = a
b += "def"
console.log(a) // abc
console.log(b) // abcdef

const firstList = [1, 2, 3]
const secondList = firstList
secondList.push(4)
console.log(firstList) // [1, 2, 3, 4]
console.log(secondList) // [1, 2, 3, 4]
