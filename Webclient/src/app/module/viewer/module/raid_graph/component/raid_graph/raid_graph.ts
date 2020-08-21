import {Component, OnDestroy, OnInit} from "@angular/core";
import {ChartDataSets, ChartOptions, ChartPoint} from 'chart.js';
import {Color} from 'ng2-charts';
import {GraphDataService} from "../../service/graph_data";
import {DataSet, is_event_data_set} from "../../domain_value/data_set";
import {DateService} from "../../../../../../service/date";
import {SettingsService} from "src/app/service/settings";
import {Subscription} from "rxjs";
import {number_to_chart_type} from "../../domain_value/chart_type";
import {get_point_style} from "../../stdlib/data_set_helper";

@Component({
    selector: "RaidGraph",
    templateUrl: "./raid_graph.html",
    styleUrls: ["./raid_graph.scss"],
    providers: [
        GraphDataService
    ]
})
export class RaidGraphComponent implements OnInit, OnDestroy {

    private subscription: Subscription;
    private subscription_events: Subscription;

    chartDataSets: Array<ChartDataSets> = [];
    chartLabels: any = [];
    chartOptions: ChartOptions = {
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
            }],
            yAxes: [{
                // stacked: true
                ticks: {
                    beginAtZero: true
                }
            }]
        },
        hover: {
            intersect: false,
            mode: "point"
        },
        animation: {
            duration: 0
        }
    };
    chartColors: Array<Color> = [
        {
            backgroundColor: 'red',
            borderColor: 'red',
        },
        {
            backgroundColor: 'blue',
            borderColor: 'blue',
        }
    ];
    chartPlugins = [];

    dataSets = [
        {id: DataSet.DamageDone, label: "Damage done"},
        {id: DataSet.DamageTaken, label: "Damage taken"},
        {id: DataSet.TotalHealingDone, label: "Total healing done"},
        {id: DataSet.TotalHealingTaken, label: "Total healing taken"},
        {id: DataSet.EffectiveHealingDone, label: "Effective healing done"},
        {id: DataSet.EffectiveHealingTaken, label: "Effective healing taken"},
        {id: DataSet.OverhealingDone, label: "Overhealing done"},
        {id: DataSet.OverhealingTaken, label: "Overhealing taken"},
        {id: DataSet.ThreatDone, label: "Threat done"},
        {id: DataSet.ThreatTaken, label: "Threat taken"},
    ];
    dataSetsSelected = [];
    selectedDataSets: Set<DataSet> = new Set();

    events = [
        {id: DataSet.Deaths, label: "Deaths"},
        {id: DataSet.Kills, label: "Kills"},
        {id: DataSet.DispelsDone, label: "Dispels done"},
        {id: DataSet.DispelsReceived, label: "Dispels received"},
    ];
    eventsSelected = [];
    selectedEvents: Set<DataSet> = new Set();

    chartTypes = [
        {value: 0, label_key: "Line Chart"},
        {value: 1, label_key: "Bar Chart"},
        {value: 2, label_key: "Scatter Chart"},
    ];
    selected_chart_type: number = 0;

    constructor(
        private graphDataService: GraphDataService,
        private dateService: DateService,
        private settingsService: SettingsService
    ) {
        this.subscription = this.graphDataService.data_points.subscribe(([x_axis, data_sets]) => {
            this.chartLabels = x_axis;
            this.chartDataSets = [];
            for (const [data_set, [real_x_axis, real_y_axis]] of data_sets) {
                const chart_points: Array<ChartPoint> = [];
                for (let i = 0; i < real_x_axis.length; ++i)
                    chart_points.push({x: real_x_axis[i], y: real_y_axis[i]});
                this.chartDataSets.push({
                    data: chart_points,
                    label: data_set,
                    type: is_event_data_set(data_set) ? "scatter" : number_to_chart_type(this.selected_chart_type),
                    pointStyle: get_point_style(data_set)
                });
            }
        });
    }

    ngOnInit(): void {
        this.selectedDataSets = new Set(this.settingsService.get_or_set("viewer_raid_graph_datasets", []));
        for (const data_set of this.selectedDataSets) {
            this.graphDataService.add_data_set(data_set);
            this.dataSetsSelected.push(this.dataSets.find(set => set.id === data_set));
        }

        this.selectedEvents = new Set(this.settingsService.get_or_set("viewer_raid_graph_events", []));
        for (const data_set of this.selectedEvents) {
            this.graphDataService.add_data_set(data_set);
            this.eventsSelected.push(this.events.find(set => set.id === data_set));
        }
    }

    ngOnDestroy(): void {
        this.subscription?.unsubscribe();
        this.subscription_events?.unsubscribe();
    }

    chartClicked({event, active}: { event: MouseEvent, active: Array<{}> }): void {
        // console.log(event, active);
    }

    chartHovered({event, active}: { event: MouseEvent, active: Array<{}> }): void {
        // console.log(event, active);
    }

    data_set_selected(data_set: any): void {
        this.selectedDataSets.add(data_set.id);
        this.graphDataService.add_data_set(data_set.id);
        this.save_selected();
    }

    data_set_deselected(data_set: any): void {
        this.selectedDataSets.delete(data_set.id);
        this.graphDataService.remove_data_set(data_set.id);
        this.save_selected();
    }

    event_selected(event: any): void {
        this.selectedEvents.add(event.id);
        this.graphDataService.add_data_set(event.id);
        this.save_selected();
    }

    event_deselected(event: any): void {
        this.selectedEvents.delete(event.id);
        this.graphDataService.remove_data_set(event.id);
        this.save_selected();
    }

    select_chart_type(chart_type: number): void {
        this.selected_chart_type = chart_type;
        this.graphDataService.update();
    }

    private save_selected(): void {
        this.settingsService.set("viewer_raid_graph_datasets", [...this.selectedDataSets.values()]);
        this.settingsService.set("viewer_raid_graph_events", [...this.selectedEvents.values()]);
    }
}
