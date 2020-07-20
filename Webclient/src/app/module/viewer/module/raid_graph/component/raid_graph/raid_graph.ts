import {Component} from "@angular/core";
import { ChartDataSets, ChartOptions } from 'chart.js';
import { Color, Label } from 'ng2-charts';

@Component({
    selector: "RaidGraph",
    templateUrl: "./raid_graph.html",
    styleUrls: ["./raid_graph.scss"]
})
export class RaidGraphComponent {

    // Array of different segments in chart
    lineChartData: Array<ChartDataSets> = [
        { data: [65, 59, 80, 81, 56, 55, 40], label: 'Product A' },
        { data: [28, 48, 40, 19, 86, 27, 90], label: 'Product B' }
    ];

    // Labels shown on the x-axis
    lineChartLabels: Array<Label> = ['January', 'February', 'March', 'April', 'May', 'June', 'July'];

    // Define chart options
    lineChartOptions: ChartOptions = {
        responsive: true,
        maintainAspectRatio: false,
        legend: {
            display: false
        },
        elements: {
            line: {
                tension: 0,
                borderWidth: 2,
                fill: false
            }
        }
    };

    // Define colors of chart segments
    lineChartColors: Array<Color> = [

        { // dark grey
            backgroundColor: 'rgba(77,83,96,0.2)',
            borderColor: 'rgba(77,83,96,1)',
        },
        { // red
            backgroundColor: 'rgba(255,0,0,0.3)',
            borderColor: 'red',
        }
    ];

    // Set true to show legends
    lineChartLegend = true;

    // Define type of chart
    lineChartType = 'line';

    lineChartPlugins = [];

    // events
    chartClicked({ event, active }: { event: MouseEvent, active: Array<{}> }): void {
        console.log(event, active);
    }

    chartHovered({ event, active }: { event: MouseEvent, active: Array<{}> }): void {
        console.log(event, active);
    }

}
