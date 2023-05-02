import { Component, OnInit } from "@angular/core";
import { Observable, of } from "rxjs";
import { Cmd } from "src/app/interfaces/cmd";
import Note from "src/app/interfaces/note";
import invokeFactory from "src/app/utils/invoke_pipe";

@Component({
  selector: "app-note-list",
  templateUrl: "./note-list.component.html",
  styles: [],
})
export class NoteListComponent implements OnInit {
  list: Observable<Note[]> = of([] as Note[]);
  ngOnInit(): void {
    this.list = invokeFactory(Cmd.BROWSE_NOTE);
  }
}
