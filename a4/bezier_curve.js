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
    svg.selectAll("#plotpoints").remove()
    svg.selectAll("#plotlines").remove()

    const count = pointSlider.property("value")
    let points = []

    // calculate point positions
    for (let i = 0; i <= count; i++) {
        points.push(bezier_func(i / count));
    }

    // draw circles for plot
    // pointer-events lets us click through
    // se we can easily click control point
    svg.selectAll("#plotpoints")
        .data(points)
        .enter()
        .append("circle")
        .attr("fill", "red")
        .attr("id", "plotpoints")
        .attr("r", 3)
        .style("pointer-events", "none")
        .attr("cx", d => d.x)
        .attr("cy", d => d.y)

    svg.selectAll("#plotlines")
        .data(points.reduce((pairs, d, i, points) => {
            // Convert list of points to pairs for line drawing
            if (i > 0) pairs.push({
                x1: points[i-1].x, y1: points[i-1].y,
                x2: d.x,           y2: d.y
            })
            return pairs
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
        // Update bezier function and curve
        d3.select("#controlpoint" + i)
            .attr("cx", d.x = d3.event.x)
            .attr("cy", d.y = d3.event.y)

        d3.select("#controltext" + i)
            .attr("x", d.x - 10)
            .attr("y", d.y - 10)
        add_plot_points_and_draw()
    }))

// draw labels
svg.selectAll("text")
    .data(control_point_pos)
    .enter()
    .append("text")
    .attr("x", d => d.x - 10)
    .attr("y", d => d.y - 10)
    .attr("id", (_, i) => "controltext" + i)
    .text((_, i) => "P" + i)

// Build the slider
pointSlider = d3.select("body")
    .append("div")
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

// shows number of plot points
const textBox = d3.select("body").append("textarea")
    .attr("readonly", "")
    .style("border", 0)
    .style("resize", "none")
    .property("value", pointSlider.property("value"))

// Draw first curve
add_plot_points_and_draw()