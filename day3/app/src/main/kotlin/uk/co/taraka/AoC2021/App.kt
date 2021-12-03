package uk.co.taraka.AoC2021
import java.io.File

fun main() {
   val input = File("../input")
       .readLines()

    println("Part1: ${part1(input)}")
}

fun part1(input: List<String>): Int {
    val counts: IntArray = input
        .map { it.toList() }
        .fold(IntArray(input[0].length)) { acc, chars ->
            chars.forEachIndexed { idx, c -> if (c == '1') { acc[idx] += 1 } };
            acc  }

    val (γ, ε) = counts.fold(Pair(0, 0)) { acc, it ->
        if (it < input.size / 2) {
            Pair(acc.first.shl(1) + 1, acc.second.shl(1))
        } else {
            Pair(acc.first.shl(1), acc.second.shl(1) + 1)
        }
    };

    return γ * ε;
}