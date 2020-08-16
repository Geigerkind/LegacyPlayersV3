export enum ChartType {
    Line = "line",
    Bar = "bar",
    Scatter = "scatter"
}

function number_to_chart_type(input: number) {
    switch (input) {
        case 0:
            return ChartType.Line;
        case 1:
            return ChartType.Bar;
        case 2:
            return ChartType.Scatter;
    }
    return ChartType.Line;
}

export {number_to_chart_type};
