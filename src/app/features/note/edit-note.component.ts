import { Component, OnInit } from "@angular/core";
import { ActivatedRoute, Router } from "@angular/router";
import { Observable, filter, map, of, skipWhile, switchMap, tap } from "rxjs";
import { Cmd } from "src/app/interfaces/cmd";
import { Note } from "src/app/interfaces/note";
import invokeFactory from "src/app/utils/invoke_pipe";

@Component({
  selector: "app-edit-note",
  templateUrl: "./edit-note.component.html",
  styles: [],
})
export class EditNoteComponent implements OnInit {
  note$: Observable<Note> = of();
  private id: string | null = null;
  constructor(private router: Router, private route: ActivatedRoute) {}
  ngOnInit(): void {
    this.note$ = this.route.queryParamMap.pipe(
      map((param) => param.get("id")),
      skipWhile((id) => !id),
      tap((id) => (this.id = id)),
      switchMap((id) => invokeFactory<Note>(Cmd.READ_NOTE, { id }))
    );
  }
  editNote(event: Note): void {
    invokeFactory(Cmd.EDIT_NOTE, {
      id: this.id,
      body: event,
    }).subscribe(() => this.router.navigateByUrl("/notes/list"));
  }
}
