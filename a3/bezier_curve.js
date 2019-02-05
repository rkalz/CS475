function generate_bezier_curve() {
    /*
    Use D3 to Query coordinates of the control points
    Using a for loop populate the coordinates of the 20 points on the
    bezier curve,
    and use D3 to select the given 20 points and set their attributes (cx,
    cy) based on the previous calculation
    */

    // Get the control points
    const pointA = d3.select("#circle0")
    const pointB = d3.select("#circle1")
    const pointC = d3.select("#circle2")
    const pointD = d3.select("#circle3")

    const bezier_func = t => {
        const newX = (1 - t) ** 3 * pointA.attr("cx")
            + (3 * (1 - t) ** 2) * t * pointB.attr("cx")
            + 3 * (1 - t) * (t ** 2) * pointC.attr("cx")
            + (t ** 3) * pointD.attr("cx")

        const newY = (1 - t) ** 3 * pointA.attr("cy")
        + (3 * (1 - t) ** 2) * t * pointB.attr("cy")
        + 3 * (1 - t) * (t ** 2) * pointC.attr("cy")
        + (t ** 3) * pointD.attr("cy")

        return [newX, newY]
    }

    const plotPoints = d3.select("g.bezier_points").selectAll("circle")
    for (let i = 0; i < plotPoints.size(); i++) {
        const point = plotPoints.nodes()[i];
        const newVals = bezier_func(i / plotPoints.size())

        point.setAttribute("cx", newVals[0])
        point.setAttribute("cy", newVals[1])
    }

    return 0
}