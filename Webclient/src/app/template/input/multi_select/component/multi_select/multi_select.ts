import {Component, EventEmitter, Input, Output} from "@angular/core";
import {IDropdownSettings} from "ng-multiselect-dropdown/multiselect.model";

@Component({
    selector: "MultiSelect",
    templateUrl: "./multi_select.html",
    styleUrls: ["./multi_select.scss"]
})
export class MultiSelectComponent {

    @Input()
    placeholder: string = 'Placeholder';
    @Input()
    dropdownList = [];
    @Input()
    dropdownSettings: IDropdownSettings = {
        singleSelection: false,
        idField: 'id',
        textField: 'label',
        selectAllText: 'Select all',
        unSelectAllText: 'Deselect all',
        itemsShowLimit: 3,
        allowSearchFilter: true
    };

    @Output()
    item_selected: EventEmitter<any> = new EventEmitter<any>();

    @Output()
    select_all: EventEmitter<any> = new EventEmitter<any>();

}
