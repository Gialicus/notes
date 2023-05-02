import { NgModule } from "@angular/core";
import { Routes, RouterModule } from "@angular/router";
import { NoteListComponent } from "./features/note/note-list.component";
import { NoteAddComponent } from "./features/note/note-add.component";

const routes: Routes = [
  { path: "", redirectTo: "notes/list", pathMatch: "full" },
  {
    path: "notes/list",
    component: NoteListComponent,
  },
  {
    path: "notes/add",
    component: NoteAddComponent,
  },
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule],
})
export class AppRoutingModule {}
