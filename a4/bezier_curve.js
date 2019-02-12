let svg;
let pointSlider;

const bezier_func = t => {
    const pointA = svg.select("#controlpoint0")
    const pointB = svg.select("#controlpoint1")
    const pointC = svg.select("#controlpoint2")
    const pointD = svg.select("#controlpoint3")

    const newX = (1 - t) ** 3 * pointA.attr("cx")
        + (3 * (1 - t) ** 2) * t * pointB.attr("cx")
        + 3 * (1 - t) * (t ** 2) * pointC.attr("cx")
        + (t ** 3) * pointD.attr("cx")

    const newY = (1 - t) ** 3 * pointA.attr("cy")
    + (3 * (1 - t) ** 2) * t * pointB.attr("cy")
    + 3 * (1 - t) * (t ** 2) * pointC.attr("cy")
    + (t ** 3) * pointD.attr("cy")

    return{ x: newX, y: newY }
}

const add_plot_points_and_draw = () => {
    // Remove old points
    svg.selectAll("#plotpoints").remove()
    svg.selectAll("#plotlines").remove()

    const count = pointSlider.property("value")
    let points = []

    // draw lines connecting circles
    for (let i = 0; i <= count; i++) {
        points.push(bezier_func(i / count));
    }

    svg.selectAll("#plotpoints")
        .data(points)
        .enter()
        .append("circle")
        .attr("fill", "red")
        .attr("id", "plotpoints")
        .attr("r", 3)
        .attr("cx", d => d.x)
        .attr("cy", d => d.y)
        .style("pointer-events", "none")

    svg.selectAll("#plotlines")
        .data(points.reduce((output, curr, i, src) => {
            if (i > 0) output.push({
                x1: src[i-1].x, y1: src[i-1].y,
                x2: curr.x,     y2: curr.y
            })
            return output
        }, []))
        .enter()
        .append("line")
        .attr("stroke", "red")
        .attr("id", "plotlines")
        .attr("x1", d => d.x1)
        .attr("y1", d => d.y1)
        .attr("x2", d => d.x2)
        .attr("y2", d => d.y2)
}

// Draw the SVG box
svg = d3.select("body").append("svg")
    .attr("width", 400)
    .attr("height", 300)
    .style("background", "pink")

const control_point_pos = [{x: 50, y: 250},
    {x: 150, y: 50}, {x: 200, y: 50}, {x: 250, y: 250}]

// Draw the control points
svg.selectAll("circle")
    .data(control_point_pos)
    .enter()
    .append("circle")
    .attr("r", 7)
    .attr("id", (_, i) => "controlpoint" + i)
    .attr("cx", d => d.x)
    .attr("cy", d => d.y)
    .call(d3.drag().on("drag", (d, i) => {
        // Move control point
        // Update bezier function and curce
        d3.select("#controlpoint" + i)
            .attr("cx", d.x = d3.event.x)
            .attr("cy", d.y = d3.event.y)
        add_plot_points_and_draw()
    }))

// Build the slider
pointSlider = d3.select("body").append("div")
    .attr("class", "slidecontainer")
    .append("input")
    .attr("type", "range")
    .attr("min", 2)
    .attr("max", 100)
    .attr("value", 3)
    .attr("class", "slider")
    .on("input", () => {
        // update bezier curve with more points
        textBox.property("value", pointSlider.property("value"))
        add_plot_points_and_draw()
    })

const textBox = d3.select("body").append("textarea")
    .attr("readonly", "")
    .style("border", 0)
    .style("resize", "none")
    .property("value", pointSlider.property("value"))

// Draw first curce
add_plot_points_and_draw()