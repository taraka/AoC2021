package uk.co.taraka.AoC2021
import java.io.File

fun main() {
   val input = File("../input")
       .readLines()

    println("Part1: ${part1(input)}")
    println("Part2: ${part2(input.map { it.toList() })}")
}

fun part1(input: List<String>): Int {
    val (γ, ε) = input
        .map { it.toList() }
        .fold(IntArray(input[0].length)) { acc, chars ->
            chars.forEachIndexed { idx, c -> if (c == '1') { acc[idx] += 1 } };
            acc  }
        .fold(Pair(0, 0)) { acc, it ->
            if (it < input.size / 2) {
                Pair(acc.first.shl(1) + 1, acc.second.shl(1))
            } else {
                Pair(acc.first.shl(1), acc.second.shl(1) + 1)
            }
        };

    return γ * ε;
}

fun part2(input: List<List<Char>>): Int {
    return co2(0, input, '1', '0') * co2(0, input, '0', '1')
}

tailrec fun co2(idx: Int, input: List<List<Char>>, a: Char, b: Char): Int {
    if (input.size == 1) {
        return Integer.parseInt(String(input[0].toCharArray()), 2)
    }
    val count: Int = input.fold(0) { acc, chars -> if (chars[idx] == '1') { acc + 1 } else { acc }}
    val common = if (count < input.size  / 2.0) { a } else { b }
    return co2(idx + 1, input.filter { it[idx] == common }, a, b);
}
