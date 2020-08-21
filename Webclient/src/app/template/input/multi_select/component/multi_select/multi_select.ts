import {Component, EventEmitter, Input, OnInit, Output, ViewChild} from "@angular/core";
import {IDropdownSettings} from "ng-multiselect-dropdown/multiselect.model";
import {AdditionalButton} from "../../domain_value/additional_button";
import set = Reflect.set;

@Component({
    selector: "MultiSelect",
    templateUrl: "./multi_select.html",
    styleUrls: ["./multi_select.scss"]
})
export class MultiSelectComponent implements OnInit {

    @ViewChild("child", {static: true}) element: any;

    @Input()
    placeholder: string = 'Placeholder';

    dropdownListData = [];

    @Input()
    set dropdownList(list: Array<any>) {
        this.dropdownListData = list;
        setTimeout(() => this.addCollectionButton(), 1000);
    }

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
        itemsShowLimit: 1,
    };

    @Input()
    additional_button: Array<AdditionalButton> = [];

    selectedItemsData: Array<any> = [];

    @Input()
    get selectedItems(): Array<any> {
        return this.selectedItemsData;
    }

    set selectedItems(items: Array<any>) {
        this.selectedItemsData = items;
        this.selectedItemsChange.emit(items);
        this.check_if_additional_buttons_are_selected();
    }

    @Output()
    item_selected: EventEmitter<any> = new EventEmitter<any>();
    @Output()
    item_deselected: EventEmitter<any> = new EventEmitter<any>();
    @Output()
    select_all: EventEmitter<any> = new EventEmitter<any>();
    @Output()
    deselect_all: EventEmitter<any> = new EventEmitter<any>();
    @Output()
    selectedItemsChange: EventEmitter<Array<any>> = new EventEmitter();

    private additional_button_checkboxes: Map<number, any> = new Map();

    ngOnInit(): void {
        this.dropdownSettings.enableCheckAll = this.enableCheckAll;
        this.dropdownSettings.allowSearchFilter = this.allowSearchFilter;
    }

    private addCollectionButton(): void {
        const collection_button_ul = this.element.__ngContext__[0].children[0].children[0].children[1].children[0];
        this.additional_button_checkboxes.clear();
        if (collection_button_ul.children.length > 0) {
            const first_child = collection_button_ul.children[0];
            first_child.style.padding = "6px 10px";
            first_child.style.borderBottom = "none";
            const last_child = collection_button_ul.children[collection_button_ul.children.length - 1];
            collection_button_ul.innerHTML = "";

            collection_button_ul.appendChild(first_child);
            for (const button of this.additional_button) {
                const clone = first_child.cloneNode(true);
                clone.children[1].innerHTML = button.label;
                clone.children[1].id = "additional_button" + button.id.toString();
                clone.children[0].checked = false;
                clone.addEventListener("click", () => {
                    clone.children[0].checked = !clone.children[0].checked;
                    this.selectedItems = button.list_selection_callback(button, this.selectedItemsData, this.dropdownListData, clone.children[0].checked);
                });
                this.additional_button_checkboxes.set(button.id, clone.children[0]);
                collection_button_ul.appendChild(clone);
            }
            collection_button_ul.children[collection_button_ul.children.length - 1].style.borderBottom = "1px solid #ccc";
            collection_button_ul.children[collection_button_ul.children.length - 1].style.paddingBottom = "12px";
            collection_button_ul.appendChild(last_child);
            this.check_if_additional_buttons_are_selected();
        }
    }

    private check_if_additional_buttons_are_selected(): void {
        for (const button of this.additional_button) {
            if (!this.additional_button_checkboxes.has(button.id))
                continue;

            const required_items = button.list_selection_callback(button, this.selectedItemsData, this.dropdownListData, true);
            const checked = required_items.every(item => this.selectedItemsData.find(r_item => r_item.id === item.id) !== undefined);
            this.additional_button_checkboxes.get(button.id).checked = checked;
        }
    }
}
