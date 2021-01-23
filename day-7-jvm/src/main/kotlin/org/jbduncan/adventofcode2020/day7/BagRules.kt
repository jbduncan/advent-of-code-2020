package org.jbduncan.adventofcode2020.day7

import com.google.common.graph.Graphs
import com.google.common.graph.ImmutableValueGraph
import com.google.common.graph.Traverser
import com.google.common.graph.ValueGraphBuilder
import java.util.*

class BagRules private constructor(private val bagRelationships: ImmutableValueGraph<String, Int>) {

  companion object {
    private val OUTER_BAG_REGEX = Regex("""^[a-z]+ [a-z]+ bag(s)?$""")
    private val NUMBER_OF_INNER_BAGS_REGEX = Regex("""^[1-9][0-9]* [a-z]+ [a-z]+ bag(s)?$""")

    @Throws(InvalidRuleException::class, RulesCycleException::class)
    fun parse(rules: String): BagRules {

      val bagRelationships = ValueGraphBuilder.directed().build<String, Int>()

      val individualRules = rules.split("\n")
      for (rule in individualRules) {
        val ruleParts = rule.split(" contain ", limit = 2)

        if (ruleParts.size != 2) {
          throw InvalidRuleException(rule)
        }
        if (!OUTER_BAG_REGEX.matches(ruleParts[0])) {
          throw InvalidRuleException(rule)
        }
        if (ruleParts[1] == "no other bags.") {
          continue
        }

        val outerBag = ruleParts[0].trimEnd('s')

        for (innerBags in ruleParts[1].trimEnd('.').split(", ")) {
          if (!NUMBER_OF_INNER_BAGS_REGEX.matches(innerBags)) {
            throw InvalidRuleException(rule)
          }
          val parts = innerBags.split(' ', limit = 2)
          val amount = parts[0].toInt()
          val innerBag = parts[1].trimEnd('.').trimEnd('s')

          bagRelationships.putEdgeValue(outerBag, innerBag, amount)
        }
      }

      RulesCycleException.throwIfCircular(bagRelationships.asGraph())

      return BagRules(ImmutableValueGraph.copyOf(bagRelationships))
    }
  }

  fun uniqueBagsContaining(bag: String): Int {
    return if (!bagRelationships.nodes().contains(bag)) {
      0
    } else {
      Traverser.forGraph(Graphs.transpose(bagRelationships)) //
          .breadthFirst(bag)
          .count() - 1
    }
  }

  fun bagsContainedBy(bag: String): Int {
    if (!bagRelationships.nodes().contains(bag)) {
      return 0
    }
    val queue: Queue<String> = ArrayDeque(listOf(bag))
    var result = 0
    while (queue.isNotEmpty()) {
      val outerBag = queue.poll()
      for (innerBag in bagRelationships.successors(outerBag)) {
        val numNestedBags = bagRelationships.edgeValue(outerBag, innerBag).orElse(0)
        result += numNestedBags
        (1..numNestedBags).forEach { _ -> queue.add(innerBag) }
      }
    }
    return result
  }
}
