package org.jbduncan.adventofcode2020.day7

import com.google.common.graph.Graph
import org.jgrapht.alg.cycle.TarjanSimpleCycles

class RulesCycleException private constructor(message: String) : UserFacingException(message) {
  companion object {
    fun <N> throwIfCircular(rulesGraph: Graph<N>) {
      val cycles =
          TarjanSimpleCycles(JgraphtUnmodifiableGraphAdapter(rulesGraph)).findSimpleCycles()
      if (cycles.isNotEmpty()) {
        val cycle = cycles.sortedBy { it.size }[0]
        val message = StringBuilder("The rules have a cycle: ")
        cycle
            .asSequence()
            .plus(cycle[0])
            .windowed(size = 2) { "${it[0]}s contain ${it[1]}s" }
            .joinTo(message, separator = ", ")
        throw RulesCycleException(message.toString())
      }
    }
  }
}
