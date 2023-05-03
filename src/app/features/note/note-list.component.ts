import { Component, OnInit } from "@angular/core";
import { Observable, of, skipWhile, switchMap, tap } from "rxjs";
import { Cmd } from "src/app/interfaces/cmd";
import { Note } from "src/app/interfaces/note";
import invokeFactory from "src/app/utils/invoke_pipe";
import {
  faCirclePlus,
  faEdit,
  faClose,
} from "@fortawesome/free-solid-svg-icons";
import { Router } from "@angular/router";
import confirmPipe from "src/app/utils/confirm";

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
  }
  goToAddNote() {
    this.router.navigateByUrl("notes/add");
  }
  goToEditNote(id: string) {
    this.router.navigateByUrl("notes/edit?id=" + id);
  }
  async deleteNote(id: string) {
    confirmPipe("Delete note with id: " + id)
      .pipe(
        skipWhile((value) => !value),
        switchMap(() => invokeFactory(Cmd.DELETE_NOTE, { id: String(id) })),
        switchMap(() => invokeFactory<Note[]>(Cmd.BROWSE_NOTE))
      )
      .subscribe((list) => (this.list = of(list)));
  }
}
