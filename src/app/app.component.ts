import { Component } from "@angular/core";
import { invoke } from "@tauri-apps/api/tauri";

@Component({
  selector: "app-root",
  templateUrl: "./app.component.html",
  styles: [],
})
export class AppComponent {
  greetingMessage = "";

  addNote(event: SubmitEvent, title: string, body: string): void {
    event.preventDefault();

    invoke<string>("add_note", { title, body }).then((text) => {
      console.log(text);
    });
    invoke<string>("browse_note").then((text) => {
      console.log(text);
    });
  }
}
