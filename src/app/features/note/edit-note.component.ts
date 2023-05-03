import { Component } from "@angular/core";
import { ActivatedRoute, Router } from "@angular/router";
import { Observable, map, switchMap } from "rxjs";
import { Cmd } from "src/app/interfaces/cmd";
import Note from "src/app/interfaces/note";
import invokeFactory from "src/app/utils/invoke_pipe";

@Component({
  selector: "app-edit-note",
  templateUrl: "./edit-note.component.html",
  styles: [],
})
export class EditNoteComponent {
  note$: Observable<Note>;
  constructor(private router: Router, private route: ActivatedRoute) {
    this.note$ = this.route.params.pipe(
      map((param) => param["id"]),
      switchMap((id) => invokeFactory<Note>(Cmd.READ_NOTE, { id }))
    );
  }
  editNote(event: Note): void {
    invokeFactory(Cmd.EDIT_NOTE, {
      title: event.title,
      body: event.body,
    }).subscribe(() => this.router.navigateByUrl("/notes/list"));
  }
}
