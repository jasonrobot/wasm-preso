import { Component } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { PrimeFinderComponent } from './prime-finder/prime-finder.component';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [RouterOutlet, PrimeFinderComponent],
  templateUrl: './app.component.html',
  styleUrl: './app.component.scss'
})
export class AppComponent {
  title = 'angular-wasm';
}
