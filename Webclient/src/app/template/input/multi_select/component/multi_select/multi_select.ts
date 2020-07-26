import {Component, EventEmitter, Input, OnInit, Output} from "@angular/core";
import {IDropdownSettings} from "ng-multiselect-dropdown/multiselect.model";

@Component({
    selector: "MultiSelect",
    templateUrl: "./multi_select.html",
    styleUrls: ["./multi_select.scss"]
})
export class MultiSelectComponent implements OnInit {

    @Input()
    placeholder: string = 'Placeholder';
    @Input()
    dropdownList = [];
    @Input()
    enableCheckAll: boolean = false;
    @Input()
    allowSearchFilter: boolean = true;
    @Input()
    dropdownSettings: IDropdownSettings = {
        idField: 'id',
        textField: 'label',
        selectAllText: 'Select all',
        unSelectAllText: 'Deselect all',
        itemsShowLimit: 3,
    };
    @Input()
    selectedItems = [];

    @Output()
    item_selected: EventEmitter<any> = new EventEmitter<any>();
    @Output()
    item_deselected: EventEmitter<any> = new EventEmitter<any>();
    @Output()
    select_all: EventEmitter<any> = new EventEmitter<any>();
    @Output()
    deselect_all: EventEmitter<any> = new EventEmitter<any>();

    ngOnInit(): void {
        this.dropdownSettings.enableCheckAll = this.enableCheckAll;
        this.dropdownSettings.allowSearchFilter = this.allowSearchFilter;
    }
}
