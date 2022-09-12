import './style';

import Router, { Route, route } from 'preact-router';
import Header from './components/header';
import Overview from './components/overview';
import Highscore from './components/highscore';
import Login from './components/login';
import AuthenticatedRoute from './components/authenticatedRoute';
import Register from './components/register';
import Error404 from './components/Error404';
import Footer from './components/footer';

export default function App() {
	return (
		<>
			<Header />
			<div class="content">
				<Router>

					<AuthenticatedRoute path="/" component={Overview} />
					<AuthenticatedRoute path="/highscore" component={Highscore} />

					<Route path="/login" component={Login} />
					<Route path="/register" component={Register} />

					<Route type="404" default component={Error404} />
				</Router>
			</div>
			<Footer />
		</>
	);
}
