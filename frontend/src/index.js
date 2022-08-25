import './style';

import Router, { Route, route } from 'preact-router';
import Header from './components/header';
import Overview from './components/overview';
import Highscore from './components/highscore';
import Login from './components/login';
import AuthenticatedRoute from './components/authenticatedRoute';
import Register from './components/register';

export default function App() {
	return (
		<div class="">
			<Header />
			<Router>

				<AuthenticatedRoute path="/" component={Overview} />
				<AuthenticatedRoute path="/highscore" component={Highscore} />

				<Route path="/login" component={Login} />
				<Route path="/register" component={Register} />
			</Router>
		</div>
	);
}
