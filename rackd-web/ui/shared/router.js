export class RouteRedirect {
    constructor(target) {
        // validate target
        this.target = target;
    }
}

export class RouteHandler {
    constructor(fn) {
        this.fn = fn;
    }
}

export class Route {
    constructor(path, handler) {
        this.path = path;
        this.tokens = Route.tokenize(path);
        this.handler = handler;
    }

    matches(target) {
        let values = Route.split(target);
        let args = new Map();
        let vit = values.values();
        let tit = this.tokens.values();
        // console.log('tokens:');
        // console.log(this.tokens);
        // console.log('values:');
        // console.log(values);

        let current_value = vit.next();
        let current_token = tit.next();
        while (!current_value.done || !current_token.done) {
            let value = current_value.value; 
            let token = current_token.value;

            if ((token && !value && !(token instanceof PathWildcard)) || (value && !token)) {
                return [false, args];
            }

            if (token instanceof PathWildcard) {
                let path = value ? [value] : [];
                let next_value;
                while (!(next_value = vit.next()).done) {
                    path.push(next_value.value);
                }
                args.set('path', path.length !== 0 ? path.join('/') : '/');
            } else if (token instanceof PathVariable) {
                args.set(token.name, value);
            } else if (token instanceof PathConstant && token.value !== value) {
                return [false, args];
            }

            current_value = vit.next();
            current_token = tit.next();
        }

        return [true, args];
    }

    static split(path) {
        return path.toLowerCase().split('/').filter(p => p);
    }

    static tokenize(path) {
        let tokens = [];
        let parts = Route.split(path);

        for (const part of parts) {
            if (part == '*') {
                tokens.push(new PathWildcard());
            } else if (part.startsWith('$')) {
                tokens.push(new PathVariable(part.slice(1)));
            } else {
                tokens.push(new PathConstant(part));
            }
        }
        return tokens;
    }
}

export class Router {
    constructor() {
        this.routes = [];
        this._fallback = undefined;
        // this.paths = new Map(); // this should be replaced with local storage to survive reloads
    }

    run() {
        this.exec(window.location.pathname);
        document.addEventListener('goto', (e) => {
            window.history.replaceState('', '', `${window.location.origin}${e.detail.path}`) 
            this.exec(e.detail.path);
        });
    }

    route(path, handler) {
        this.routes.push(new Route(path, new RouteHandler(handler)));
        return this;
    }

    redirect(path, target) {
        this.routes.push(new Route(path, new RouteRedirect(target)));
        return this;
    }

    fallback(handler) {
        this._fallback = new RouteHandler(handler);
        return this;
    }

    exec(target) {
        for (let route of this.routes) {
            let [matches, args] = route.matches(target);
            // console.log(`testing route:`);
            // console.log(route);
            if (matches && route.handler instanceof RouteHandler) {
                return route.handler.fn(Object.fromEntries(args));
            } 
            if (matches && route.handler instanceof RouteRedirect) {
                let path = route.handler.target;
                for (let [key, value] of args) {
                    path = path.replace(`$${key}`, value);
                }
                window.history.replaceState('', '', `http://${window.location.host}${path}`);
                return this.exec(window.location.pathname);
            }
        }

        this._fallback?.fn();
    }
}

// export class Path {
//     constructor(path) {
//         this.path = path;
//     }
// }

export class PathVariable {
    constructor(name) {
        this.name = name;
    }
    get value() { return this._value; }
    set value(v) { this._value = v; }
}

export class PathConstant {
    constructor(value) {
        this._value = value
    }

    get value() { return this._value; }
}

export class PathWildcard {
    constructor() { }
}

export function goto(path) {
    // let url = URL(path, window.location.origin);
    let target = navigate(window.location.pathname, path);
    console.log(`navigating to: ${target}`);
    document.dispatchEvent(new CustomEvent('goto', { detail: { path: target }}));
  
    function navigate(from, to) {
      if (!from || !to)
        return;
      if (to && to.startsWith('/'))
        return to;
      
      let fromParts = from.toLowerCase().split('/').filter(p => p);
      let toParts = to.toLowerCase().split('/').filter(p => p);
      for (let part of toParts) {
        switch (part) {
          case '..': 
            fromParts.pop();
            break;
          case '.':
            continue;
          default: 
            fromParts.push(part);
            break;
        }
      }
      return `/${fromParts.join('/')}`;  
    }
  }
  