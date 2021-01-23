package org.jbduncan.adventofcode2020.day7

import com.google.common.graph.EndpointPair
import com.google.common.graph.Graph
import org.jgrapht.graph.guava.BaseGraphAdapter

class JgraphtUnmodifiableGraphAdapter<N>(graph: Graph<N>) : BaseGraphAdapter<N, Graph<N>>(graph) {
  override fun addEdge(sourceVertex: N?, targetVertex: N?): EndpointPair<N> {
    throw IllegalStateException("graph is not mutable")
  }

  override fun addEdge(sourceVertex: N?, targetVertex: N?, e: EndpointPair<N>?): Boolean {
    throw IllegalStateException("graph is not mutable")
  }

  override fun addVertex(): N {
    throw IllegalStateException("graph is not mutable")
  }

  override fun addVertex(v: N?): Boolean {
    throw IllegalStateException("graph is not mutable")
  }

  override fun removeEdge(sourceVertex: N?, targetVertex: N?): EndpointPair<N> {
    throw IllegalStateException("graph is not mutable")
  }

  override fun removeEdge(e: EndpointPair<N>?): Boolean {
    throw IllegalStateException("graph is not mutable")
  }

  override fun removeVertex(v: N?): Boolean {
    throw IllegalStateException("graph is not mutable")
  }

  override fun setEdgeWeight(e: EndpointPair<N>?, weight: Double) {
    throw IllegalStateException("graph is not mutable")
  }
}
