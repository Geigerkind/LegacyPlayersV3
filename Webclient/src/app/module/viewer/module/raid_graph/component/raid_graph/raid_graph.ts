import {Component, OnInit} from "@angular/core";
import {ChartDataSets, ChartOptions, ChartPoint} from 'chart.js';
import {Color} from 'ng2-charts';
import {GraphDataService} from "../../service/graph_data";
import {DataSet} from "../../domain_value/data_set";
import {DateService} from "../../../../../../service/date";
import {SettingsService} from "src/app/service/settings";

@Component({
    selector: "RaidGraph",
    templateUrl: "./raid_graph.html",
    styleUrls: ["./raid_graph.scss"],
    providers: [
        GraphDataService
    ]
})
export class RaidGraphComponent implements OnInit {

    // Array of different segments in chart
    lineChartData: Array<ChartDataSets> = [];

    // Labels shown on the x-axis
    lineChartLabels: any = [];

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
                borderWidth: 1,
                fill: false
            },
            point: {
                borderWidth: 1,
                radius: 2,
                hoverRadius: 10
            }
        },
        scales: {
            xAxes: [{
                ticks: {
                    autoSkip: true,
                    autoSkipPadding: 25,
                    callback: (value, index, values) => {
                        return this.dateService.toRPLLTime(value);
                    }
                }
            }]
        }
    };

    // Define colors of chart segments
    lineChartColors: Array<Color> = [
        {
            backgroundColor: 'red',
            borderColor: 'red',
        },
        {
            backgroundColor: 'blue',
            borderColor: 'blue',
        }
    ];

    // Set true to show legends
    lineChartLegend = true;

    // Define type of chart
    lineChartType = 'line';

    lineChartPlugins = [];

    dataSets = [
        {id: DataSet.DamageDone, label: "Damage done"},
        {id: DataSet.DamageTaken, label: "Damage taken"},
    ];
    dataSetsSelected = [];

    selected: Set<DataSet> = new Set();

    constructor(
        private graphDataService: GraphDataService,
        private dateService: DateService,
        private settingsService: SettingsService
    ) {
        this.graphDataService.data_points.subscribe(([x_axis, data_sets]) => {
            this.lineChartLabels = x_axis;
            this.lineChartData = [];
            for (const [data_set, [real_x_axis, real_y_axis]] of data_sets) {
                const chart_points: Array<ChartPoint> = [];
                for (let i = 0; i < real_x_axis.size; ++i)
                    chart_points.push({x: real_x_axis[i], y: real_y_axis[i]});

                this.lineChartData.push({
                    data: chart_points,
                    label: data_set
                });
            }
        });
    }

    ngOnInit(): void {
        this.selected = new Set(this.settingsService.get_or_set("viewer_raid_graph_datasets", []));
        for (const data_set of this.selected) {
            this.graphDataService.add_data_set(data_set);
            this.dataSetsSelected.push(this.dataSets.find(set => set.id === data_set));
        }
    }

    // events
    chartClicked({event, active}: { event: MouseEvent, active: Array<{}> }): void {
        console.log(event, active);
    }

    chartHovered({event, active}: { event: MouseEvent, active: Array<{}> }): void {
        console.log(event, active);
    }

    data_set_selected(data_set: any): void {
        this.selected.add(data_set.id);
        this.graphDataService.add_data_set(data_set.id);
        this.save_selected();
    }

    data_set_deselected(data_set: any): void {
        this.selected.delete(data_set.id);
        this.graphDataService.remove_data_set(data_set.id);
        this.save_selected();
    }

    private save_selected(): void {
        this.settingsService.set("viewer_raid_graph_datasets", [...this.selected.values()]);
    }
}
