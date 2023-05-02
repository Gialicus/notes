import { Component, OnInit } from "@angular/core";
import { Observable, of } from "rxjs";
import { Cmd } from "src/app/interfaces/cmd";
import Note from "src/app/interfaces/note";
import invokeFactory from "src/app/utils/invoke_pipe";
import {
  faCirclePlus,
  faEdit,
  faClose,
} from "@fortawesome/free-solid-svg-icons";
import { Router } from "@angular/router";

@Component({
  selector: "app-note-list",
  templateUrl: "./note-list.component.html",
  styles: [],
})
export class NoteListComponent implements OnInit {
  list: Observable<Note[]> = of([] as Note[]);
  addIcon = faCirclePlus;
  editIcon = faEdit;
  deleteIcon = faClose;
  constructor(private router: Router) {}
  ngOnInit(): void {
    this.list = invokeFactory(Cmd.BROWSE_NOTE);
    console.log(this.router.url);
  }
  goToAddNote() {
    this.router.navigateByUrl("notes/add").then(console.log);
  }
}
