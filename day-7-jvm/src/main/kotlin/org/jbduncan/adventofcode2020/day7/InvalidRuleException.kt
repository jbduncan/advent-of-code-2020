package org.jbduncan.adventofcode2020.day7

class InvalidRuleException(rule: String) :
    UserFacingException(INVALID_RULE_ERROR_MESSAGE_FORMAT.format(rule)) {
  companion object {
    private val INVALID_RULE_ERROR_MESSAGE_FORMAT =
        """
          The rule:
              "%s"
          does not have the format:
              "<adjective> <colour> bags contain <number> <adjective> <colour> (bag|bags)."
        """.trimIndent()
  }
}
