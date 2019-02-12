let svg;
let pointSlider;

window.onload = () => {
    svg = d3.select("body").append("svg")
        .attr("width", 400)
        .attr("height", 300)
        .style("background", "pink")

    pointSlider = d3.select("body").append("div")
        .attr("class", "slidecontainer")
        .append("input")
        .attr("type", "range")
        .attr("min", 2)
        .attr("max", 100)
        .attr("value", 3)
        .attr("class", "slider")

    const textBox = d3.select("body").append("textarea")
        .attr("readonly", "")
        .style("border", 0)
        .style("resize", "none")
        .property("value", pointSlider.property("value"))

    pointSlider.on("input", () => {
        textBox.property("value", pointSlider.property("value"))
    })
}