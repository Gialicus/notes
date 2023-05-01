import { Component, ElementRef, OnInit, ViewChild } from "@angular/core";
import { Observable, of } from "rxjs";
import Note from "./interfaces/note";
import invokeFactory from "./utils/invoke_pipe";
import { Cmd } from "./interfaces/cmd";
import { ThemeSelectorService } from "./shared/theme-selector.service";

@Component({
  selector: "app-root",
  templateUrl: "./app.component.html",
  styles: [],
})
export class AppComponent implements OnInit {
  @ViewChild("thema") thema?: ElementRef;
  list: Observable<Note[]> = of([] as Note[]);
  constructor(public themeSelector: ThemeSelectorService) {}
  ngOnInit(): void {
    this.list = invokeFactory(Cmd.BROWSE_NOTE);
    this.themeSelector.theme$.subscribe((val) =>
      this.thema?.nativeElement.setAttribute("data-theme", val)
    );
  }
  addNote(event: SubmitEvent, title: string, body: string): void {
    event.preventDefault();
    invokeFactory(Cmd.ADD_NOTE, { title, body }).subscribe();
  }
  changeTheme(e: any) {
    e.preventDefault();
    if (e && e.target && e.target.value) {
      this.themeSelector.theme = e.target.value;
    }
  }
}
