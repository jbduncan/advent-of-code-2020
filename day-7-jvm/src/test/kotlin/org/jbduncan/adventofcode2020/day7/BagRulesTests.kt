package org.jbduncan.adventofcode2020.day7

import com.google.common.truth.Truth.assertThat
import org.junit.jupiter.api.Nested
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.assertThrows

class BagRulesTests {

  @Nested
  inner class GivenBrightWhiteBagsContainOneShinyGoldBag {

    @Nested
    inner class WhenFindingTheNumberOfUniqueBagsThatContainShinyGoldBags {

      @Test
      fun `then the result is 1`() {
        val rules = "bright white bags contain 1 shiny gold bag."

        val result = BagRules.parse(rules).uniqueBagsContaining("shiny gold bag")

        assertThat(result).isEqualTo(1)
      }
    }

    @Nested
    inner class WhenFindingTheNumberOfBagsThatAShinyGoldBagContains {

      @Test
      fun `then the result is 0`() {
        val rules = "bright white bags contain 1 shiny gold bag."

        val result = BagRules.parse(rules).bagsContainedBy("shiny gold bag")

        assertThat(result).isEqualTo(0)
      }
    }

    @Nested
    inner class WhenFindingTheNumberOfBagsThatABrightWhiteBagContains {

      @Test
      fun `then the result is 1`() {
        val rules = "bright white bags contain 1 shiny gold bag."

        val result = BagRules.parse(rules).bagsContainedBy("bright white bag")

        assertThat(result).isEqualTo(1)
      }
    }
  }

  @Nested
  inner class GivenBrightWhiteBagsContainTwoShinyGoldBags {

    @Nested
    inner class WhenFindingTheNumberOfUniqueBagsThatContainShinyGoldBags {

      @Test
      fun `then the result is 1`() {
        val rules = "bright white bags contain 2 shiny gold bags."

        val result = BagRules.parse(rules).uniqueBagsContaining("shiny gold bag")

        assertThat(result).isEqualTo(1)
      }
    }

    @Nested
    inner class WhenFindingTheNumberOfBagsThatABrightWhiteBagContains {

      @Test
      fun `then the result is 2`() {
        val rules = "bright white bags contain 2 shiny gold bags."

        val result = BagRules.parse(rules).bagsContainedBy("bright white bag")

        assertThat(result).isEqualTo(2)
      }
    }
  }

  @Nested
  inner class GivenMutedYellowBagsContainTwoShinyGoldBagsAndNineFadedBlueBags {

    @Nested
    inner class WhenFindingTheNumberOfUniqueBagsThatContainShinyGoldBags {

      @Test
      fun `then the result is 1`() {
        val rules = "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags."

        val result = BagRules.parse(rules).uniqueBagsContaining("shiny gold bag")

        assertThat(result).isEqualTo(1)
      }
    }

    @Nested
    inner class WhenFindingTheNumberOfBagsThatAMutedYellowBagContains {

      @Test
      fun `then the result is 11`() {
        val rules = "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags."

        val result = BagRules.parse(rules).bagsContainedBy("muted yellow bag")

        assertThat(result).isEqualTo(11)
      }
    }
  }

  @Nested
  inner class GivenLightRedBagsContainOneBrightWhiteBag {

    @Nested
    inner class AndBrightWhiteBagsContainOneShinyGoldBag {

      @Nested
      inner class WhenFindingTheNumberOfUniqueBagsThatContainShinyGoldBags {

        @Test
        fun `then the result is 2`() {
          val rules =
              """
                light red bags contain 1 bright white bag.
                bright white bags contain 1 shiny gold bag.
              """.trimIndent()

          val result = BagRules.parse(rules).uniqueBagsContaining("shiny gold bag")

          assertThat(result).isEqualTo(2)
        }
      }

      @Nested
      inner class WhenFindingTheNumberOfBagsThatLightRedBagsContain {

        @Test
        fun `then the result is 2`() {
          val rules =
              """
                light red bags contain 1 bright white bag.
                bright white bags contain 1 shiny gold bag.
              """.trimIndent()

          val result = BagRules.parse(rules).bagsContainedBy("light red bag")

          assertThat(result).isEqualTo(2)
        }
      }
    }
  }

  @Nested
  inner class GivenFadedBlueBagsContainNoOtherBags {

    @Nested
    inner class WhenFindingTheNumberOfUniqueBagsThatContainShinyGoldBags {

      @Test
      fun `then the result is 0`() {
        val rules = "faded blue bags contain no other bags."

        val result = BagRules.parse(rules).uniqueBagsContaining("shiny gold bag")

        assertThat(result).isEqualTo(0)
      }
    }

    @Nested
    inner class WhenFindingTheNumberOfBagsThatFadedBlueBagsContain {

      @Test
      fun `then the result is 0`() {
        val rules = "faded blue bags contain no other bags."

        val result = BagRules.parse(rules).bagsContainedBy("faded blue bags")

        assertThat(result).isEqualTo(0)
      }
    }
  }

  @Nested
  inner class GivenShinyGoldBagsContain2DarkRedBags {

    @Nested
    inner class AndDarkRedBagsContain2DarkOrangeBags {

      @Nested
      inner class WhenFindingTheNumberOfBagsThatShinyGoldBagsContain {

        @Test
        fun `then the result is 6`() {
          val rules =
              """
                shiny gold bags contain 2 dark red bags.
                dark red bags contain 2 dark orange bags.
                dark orange bags contain no other bags.
              """.trimIndent()

          val result = BagRules.parse(rules).bagsContainedBy("shiny gold bag")

          assertThat(result).isEqualTo(6)
        }
      }

      @Nested
      inner class AndDarkOrangeBagsContain2YellowBags {

        @Nested
        inner class WhenFindingTheNumberOfBagsThatShinyGoldBagsContain {

          @Test
          fun `then the result is 14`() {
            val rules =
                """
                  shiny gold bags contain 2 dark red bags.
                  dark red bags contain 2 dark orange bags.
                  dark orange bags contain 2 dark yellow bags.
                  dark yellow bags contain no other bags.
                """.trimIndent()

            val result = BagRules.parse(rules).bagsContainedBy("shiny gold bag")

            assertThat(result).isEqualTo(14)
          }
        }
      }
    }
  }

  @Nested
  inner class GivenARuleWhichDoesNotDescribeABagContainingAnotherBag {

    @Nested
    inner class WhenFindingTheNumberOfUniqueBagsThatContainShinyGoldBags {

      @Test
      fun `then an exception is thrown`() {
        val rules = "loud purple bags play bagpipes."

        val codeUnderTest: () -> Unit = { BagRules.parse(rules) }

        val exception = assertThrows<InvalidRuleException>(codeUnderTest)
        assertThat(exception.message).contains("rule")
      }
    }
  }

  @Nested
  inner class GivenARuleWithAnOuterBagThatIsMissingAColour {

    @Nested
    inner class WhenFindingTheNumberOfUniqueBagsThatContainShinyGoldBags {

      @Test
      fun `then an exception is thrown`() {
        val rules = "loud bags contain 1 crazy maroon bag."

        val codeUnderTest: () -> Unit = { BagRules.parse(rules) }

        val exception = assertThrows<InvalidRuleException>(codeUnderTest)
        assertThat(exception.message).contains("rule")
      }
    }
  }

  @Nested
  inner class GivenARuleWithAnInnerBagThatIsMissingAColour {

    @Nested
    inner class WhenFindingTheNumberOfUniqueBagsThatContainShinyGoldBags {

      @Test
      fun `then an exception is thrown`() {
        val rules = "loud purple bags contain 1 crazy bag."

        val codeUnderTest: () -> Unit = { BagRules.parse(rules) }

        val exception = assertThrows<InvalidRuleException>(codeUnderTest)
        assertThat(exception.message).contains("rule")
      }
    }
  }

  @Nested
  inner class GivenTwoRulesThatReferToEachOther {

    @Nested
    inner class WhenFindingTheNumberOfUniqueBagsThatContainShinyGoldBags {

      @Test
      fun `then an exception about the cycle is thrown`() {
        val rules =
            """
              loud purple bags contain 1 crazy maroon bag.
              crazy maroon bags contain 1 loud purple bag.
            """.trimIndent()

        val codeUnderTest: () -> Unit = { BagRules.parse(rules) }

        val exception = assertThrows<RulesCycleException>(codeUnderTest)
        with(exception) {
          assertThat(message).contains("cycle")
          assertThat(message).containsMatch("loud purple bag.+crazy maroon bag.+loud purple bag")
        }
      }
    }
  }

  @Nested
  inner class GivenANumberOfRulesFormTwoCycles {

    @Nested
    inner class WhenFindingTheNumberOfUniqueBagsThatContainShinyGoldBags {

      @Test
      fun `then an exception about the shortest cycle is thrown`() {
        val rules =
            """
              loud purple bags contain 1 bold blue bag, 1 crazy maroon bag.
              crazy maroon bags contain 1 loud purple bag.
              bold blue bags contain 1 audacious orange bag.
              audacious orange bags contain 1 loud purple bag.
            """.trimIndent()

        val codeUnderTest: () -> Unit = { BagRules.parse(rules) }

        val exception = assertThrows<RulesCycleException>(codeUnderTest)
        with(exception) {
          assertThat(message).contains("cycle")
          assertThat(message).containsMatch("loud purple bag.+crazy maroon bag.+loud purple bag")
          assertThat(message).doesNotContain("bold blue bag")
          assertThat(message).doesNotContain("audacious orange bag")
        }
      }
    }
  }
}
