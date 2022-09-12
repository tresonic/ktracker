import '../header'

import { Link, route } from "preact-router";
import authService from "../services/auth.service";

export default function Header() {

    const onLogout = e => {
        authService.logout();
        route("/login");
    }

    const onMenuButton = e => {
        document.querySelector("#menu-button").classList.toggle("is-active");
        document.querySelector("#navbar").classList.toggle("is-active");
    }

    return (
        <header>

            <nav role="navigation" aria-label="main navigation">
                <div class="brand"><Link class="brandcombo" href="/"><img class="icon" src="assets/favicon.ico" alt="icon"/>&nbsp;kTracker</Link></div>
                <ul>
                    <li><Link class="navitem" href="/highscore">Highscore</Link></li>
                    <li><Link class="navitem" href="/">Über</Link></li>
                    <li><div class="navitem" onClick={onLogout}>Logout</div></li>
                </ul>
                {/* <div class="navbar-brand">
                <Link className="navbar-item" href="/">kTracker</Link>

                <a id="menu-button" role="button" class="navbar-burger" onClick={onMenuButton} aria-label="menu" aria-expanded="false" data-target="navbar">
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                </a>
            </div>
            
            <div id="navbar" class="navbar-menu">
                <div class="navbar-start">
                    <Link className="navbar-item" href="/highscore">Highscore</Link>
                </div>

                <div class="navbar-end">
                    <div class="navbar-item"><button class="button is-primary" onClick={onLogout}>Logout</button></div>
                </div>
            </div> */}
            </nav>
        </header>
    );
}