import { NgModule } from "@angular/core";
import { BrowserModule } from "@angular/platform-browser";
import { AppComponent } from "./app.component";
import { SharedModule } from "./shared/shared/shared.module";
import { AppRoutingModule } from "./app-routing.module";
import { NoteModule } from "./features/note/note.module";

@NgModule({
  declarations: [AppComponent],
  imports: [BrowserModule, SharedModule, AppRoutingModule, NoteModule],
  providers: [],
  bootstrap: [AppComponent],
})
export class AppModule {}
