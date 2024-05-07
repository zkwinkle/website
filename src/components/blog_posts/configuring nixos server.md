Using NixOS to set up my Linode server was very frustrating for the first few hours, and extremely satisfactory once it was done.
I ***highly recommend it***, so I wanna share my experience hoping it'll make someone else's initial experience with NixOS easier.

As mentioned, I'm using Linode as my VPS provider but of course the actual NixOS configuration part can be done anywhere where you can install the distro.

*Disclaimer:* I feel like I made this guide too detailed, feel free to skip sections if it gets too slow for you. Now, with that considered, I'd say the guide is also very short for what it achieves. It **will** leave you with a really strong base to work with for your server.

---

You'll be able to find all the code examples I mention [in my repo](https://github.com/zkwinkle/website-server/tree/82a3738db00184965c14029a8977506780003b80).
But I'll link them throughout the guide anyways.

# Linode setup

As of writing, Linode doesn't officially support NixOS, but they have a really nice [guide](https://www.linode.com/docs/guides/install-nixos-on-linode/) on installing it anyways.
It will leave you with a base NixOS instance that you'll be free to bootstrap any config into.

# First, docs

If you haven't, get familiar with the [nix language](https://nix.dev/tutorials/nix-language).

Now, finding the right documentation can be challenging at first with Nix, so here's the main useful resources for NixOS IMO:

- [NixOS option search](https://search.nixos.org/options): This site let's you search through all the configuration options for NixOS and is hands down the most useful resource to have handy.
- [NixOS manual](https://nixos.org/manual/nixos/stable/): RTFM.
- [Your preferred search engine](https://duckduckgo.com/): Mostly to find NixOS discourse forum posts or NixOS wiki pages.

# Basic OS setup

You can skip straight to the the next section if you're already familiar with
Nix and basic system config like users and SSH.
I'm writing this section to be thorough and able to fully guide a newbie (like I was) into the topic.

## Using nix flakes

I'll make this short because I don't fully grok flakes yet.
I will simply state that [nix flakes are good](https://nixos-and-flakes.thiscute.world/nixos-with-flakes/introduction-to-flakes#introduction-to-flakes), so I wanted to set up my OS config as one.

Here's the full file: [`flake.nix`](https://github.com/zkwinkle/website-server/blob/82a3738db00184965c14029a8977506780003b80/flake.nix)

I'll go over the "highlights".

#### Overlay for nixpkgs:

Here we're creating a copy of nixpkgs (which we'll end up using instead of nixpkgs) that has an *overlay* applied to add our custom packages to the system.

```nix
pkgs = import nixpkgs {
  inherit system;
  overlays = [ (import ./overlay.nix) ];
};
```

#### Defining our flake

```nix
NixOSConfigurations.website-server = nixpkgs.lib.nixosSystem {
	inherit pkgs system;
	modules = [ ./configuration ];
};
```

This is telling the flake we have one configuration called `website-server`, where `system="x86_64-linux"` and we'll use our overlayed `pkgs` instead.

We could add more configurations with different names if we felt like it by adding the same thing but replacing `website-server` with something else.

## Configuration structure

The flake will look inside the [`configuration/`](https://github.com/zkwinkle/website-server/blob/82a3738db00184965c14029a8977506780003b80/configuration) folder for our config. There, it will look for a [`default.nix`](https://github.com/zkwinkle/website-server/blob/82a3738db00184965c14029a8977506780003b80/configuration/default.nix) file, where we just include all the other modules.

## Basic config

The boot options inside [`boot.nix`](https://github.com/zkwinkle/website-server/blob/82a3738db00184965c14029a8977506780003b80/configuration/boot.nix), as well as the user options inside [`configuration.nix`](https://github.com/zkwinkle/website-server/blob/82a3738db00184965c14029a8977506780003b80/configuration/boot.nix) are covered inside the [Linode guide](https://www.linode.com/docs/guides/install-nixos-on-linode/).
You can also look for the options in the [NixOS search index](https://search.nixos.org/options) to understand them.

# Network setup

I had a constant theme with using NixOS, at first I had trouble getting off the ground and just figuring out where to start; but once I found the right options and their docs, everything came together incredibly easy.

We'll see this first with the basic network setup.

From the [`net.nix`](https://github.com/zkwinkle/website-server/blob/82a3738db00184965c14029a8977506780003b80/configuration/net.nix) file:


```nix
networking = {
	usePredictableInterfaceNames = false;
	firewall.allowedTCPPorts = [
		80 # http
		443 # https
		22 # SSH
	];
	hostName = "website-server";
	useDHCP = false;
	interfaces.eth0.useDHCP = true;
};
```

This tiny block of code gives us everything we need to setup our network.
Setting the `networking.firewall.allowedTCPPorts` is all we need to setup our firewall with only the necessary ports exposed. The other options are just the hostname and necessary settings according to the [Linode guide](https://www.linode.com/docs/guides/install-nixos-on-linode/).

## SSH setup

```nix
services.openssh = {
	enable = true;
	settings = {
		PermitRootLogin = "no";
		PasswordAuthentication = false;
	};
};
```

This is all we need to setup SSH while disabling root login and password authentication. Pretty striaghtforward.

Additionally, the following line in our [user config](https://github.com/zkwinkle/website-server/blob/82a3738db00184965c14029a8977506780003b80/configuration/confgiuration.nix) gives the non-root user I created my public SSH key. Which is necessary so I can log in without inputting a password.

```nix
openssh.authorizedKeys.keys = [ "ssh-ed25519 AAAA... igna@waterfall" ];
```

On the [README](https://github.com/zkwinkle/website-server/blob/82a3738db00184965c14029a8977506780003b80/README.md?plain=1#L35-L63) there's an explanation on how to set this up (create key pairs).

# Packaging my website

This website is served by a Rust Axum server that's doing SSR, you can find its repo [here](https://github.com/zkwinkle/website).
To add this website to my NixOS server I had to package it with Nix.
This just entails adding a [`default.nix`](https://github.com/zkwinkle/website/blob/875efa7d60639e4fb2e09a19de7ae8c838a4f656/default.nix) to the website's repo which holds a package "derivation".

Thankfully, nix has [great support](https://ryantm.github.io/nixpkgs/languages-frameworks/rust/#compiling-rust-applications-with-cargo) for packaging Rust applications so it's incredibly easy.
The only highlight here is that I needed to include in the installation a `public/` folder which holds my website's static files.
My website's Rust server check's an env var `PUBLIC_DIR` to find it, so I added the following to the [`default.nix`](https://github.com/zkwinkle/website/blob/875efa7d60639e4fb2e09a19de7ae8c838a4f656/default.nix#L11-L15):

```
postInstall = ''
            cp -r public $out/public
            wrapProgram $out/bin/website \
            --set PUBLIC_DIR "$out/public"
            '';
```

- [`postInstall`](https://nixos.org/manual/nixpkgs/stable/#ssec-install-phase) is a hook where you can run some commands after installation has finished.

- The `$out` variable refers to the path in the nix store where the package will be installed, which usually looks like `/nix/store/r4gac4p8mxgrcl749fa9lim2alkyn440-website-0.1`.

Here we're copying the `public/` folder to the installed location and using [`wrapProgram`](https://ryantm.github.io/nixpkgs/stdenv/stdenv/#fun-wrapProgram) so that whenever `website` is called it has the `PUBLIC_DIR` env variable set to the corresponding path.

# Nginx setup

Okay, so far we have setup a basic OS that can run our website's server. Now, I didn't want to expose this server directly because

1. I might want to host more stuff on this NixOS instance in the future, so I want a reverse proxy.
2. Out of lazyness I don't wanna figure out coding up up TLS/HTTPS on my Rust server.

Sorry if you're tired of hearing my praises by now but, thankfully, setting up [NGINX](https://nginx.com/) on NixOS is incredibly easy 🙏🏻.

First, here's a basic setup to redirect connections to the port our Axum server is running on:

```
services.nginx = {
	enable = true;

	virtualHosts."zkwinkle.is-a.dev" = {
		locations."/" = {
			proxyPass = "http://0.0.0.0:31415";
			proxyWebsockets = true;
		};
	};
};
```

## NGINX with SSL/TLS

Now, I also want my NGINX server to redirect *http* requests to *https* and handle the TLS certificates. Additionally, I want it to refresh the certificates automatically.

This endeavour requires a whole ***7!!!*** lines of additional configuration code 😰. So hard!

(Marked with `<----`.)

```nix
services.nginx = {
  enable = true;

  virtualHosts."zkwinkle.is-a.dev" = {
    forceSSL = true;   # <---- Redirects http -> https
    enableACME = true; # <---- Ask for a certificate for this host
    root = "/var/www/zkwinkle.is-a.dev"; # <----
    locations."/" = {
      proxyPass = "http://0.0.0.0:31415";
      proxyWebsockets = true;
    };
  };
};

# Handle certificates through ACME protocol.
# Uses Let's Encrypt as a default provider.
security.acme = {                       # <----
  acceptTerms = true;                   # <----
  defaults.email = "ignaevc@gmail.com"; # <----
};                                      # <----
```

See the [manual](https://nixos.org/manual/nixos/stable/#module-security-acme) for more complex use cases.

## Best practices

Configuring NGINX's best practices and optimizations is just 4 more lines:

```
services.nginx = {
  recommendedGzipSettings = true;
  recommendedProxySettings = true;
  recommendedTlsSettings = true;
  recommendedOptimisation = true;
}
```

# systemd service for our website

This is the last part of the [`net.nix`](https://github.com/zkwinkle/website-server/blob/82a3738db00184965c14029a8977506780003b80/configuration/net.nix)  file that we haven't talked about.

I'll just leave here the code block that defines the systemd service that automatically starts the website's Rust server, without much explanation. All the settings are better explained by systemd service docs. I recommend [Arch Linux's wiki section on unit files](https://wiki.archlinux.org/title/Systemd#Writing_unit_files).

```nix
systemd.services.website = {
  enable = true;
  description = "My own personal website";

  after = [ "network.target" "network-online.target" "nss-lookup.target" ];
  requires = [ "network.target" ];
  wants = [ "network-online.target" ];

  serviceConfig = {
    ExecStart = "${pkgs.website}/bin/website";
    Type = "simple";
    Restart = "always";
  };
  wantedBy = [ "multi-user.target" ];
};
```

I will only explain that the variable `${pkgs.<package>}` holds the path to a package's nix store installation folder.
Like the `$out` variable when we were packaging our program.

We can access this variable for our custom `website` package thanks to our overlay that added `website` to the `pkgs`.

# Deployment

I previously mentioned we're using an overlay to add our packages to nixpkgs so that we can later refer to them in our NixOS configuration.

All [that overlay](https://github.com/zkwinkle/website-server/blob/82a3738db00184965c14029a8977506780003b80/overlay.nix) does is call our package's derivation in order to load it:

```nix
website = self.callPackage ./website { };
```

The way that line is written means that it's expecting our website's package (the Rust source code + `default.nix` file) to be inside a directory called `website`.


Previously, I was using git submodules to pull in my website's package repo into my NixOS config repo. But that meant that after changing my website I had to:

1. Pull in the submodule's changes in my NixOS config repo.
2. Commit the submodule's update and push it.
3. Pull the changes or clone the NixOS config inside my Linode instance through SSH.
4. Finally run `nixos-rebuild switch --flake .#website-server` to update the website.

This is obviously a hassle, and at this point it's much better to use Nix's [`fetchFromGithub`](https://ryantm.github.io/nixpkgs/builders/fetchers/#fetchfromgithub). But I didn't wanna use a fetcher either because it takes in a hash as a parameter, so it's a similar issue to using the submodule; I'd have to update the NixOS config repo to point to the new website updates.

## The solution

The best way I found to update the system pulling in the latest website changes, was to just clone the website's repo as a subdir and call it locally.

Of course, I made [a script](https://github.com/zkwinkle/website-server/blob/82a3738db00184965c14029a8977506780003b80/update-website/update-website) to automate this process.
And created a Nix package derivation for it and added it as a package to my OS.

*Disclaimer:* A Nix purist might complain that this makes my config "impure", and it does. But I simply much prefer to easily pull in updates to my website than keeping my server's OS config 100% pure and reproducible.

Last thing I did was add an alias to run the following:

```sh
alias deploy='ssh website-server -t update-website'
```

---

Now, I just run `deploy` from my laptop's terminal, and in just that command I update my VPS' OS, pulling in my latest website changes, only asking for a password to run a `sudo` command.

On top of that, I'm left with an incredibly solid base for a NixOS configuration that I can super easily expand in the future to include any additional services I wanna host.

> Grande es Nix!
