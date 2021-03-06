---
title: Install Vector On NixOS
sidebar_label: NixOS
description: Install Vector on NixOS
---

This document will cover installing Vector on NixOS.

## Install

<Tabs
block={true}
defaultValue="daemon"
values={[{"label":"As a Daemon","value":"daemon"}]}>
<TabItem value="daemon">

The [daemon deployment strategy][docs.strategies#daemon] is designed for data
collection on a single host. Vector runs in the background, in its own process,
collecting _all_ data for that host.
Typically data is collected from a process manager, such as Journald via
Vector's [`journald` source][docs.sources.journald], but can be collected
through any of Vector's [sources][docs.sources].
The following diagram demonstrates how it works.

<DaemonDiagram
  platformName={null}
  sourceName={null}
  sinkName={null} />

---

<Tabs
centered={true}
className={"rounded"}
defaultValue={"nix"}
placeholder="Please choose an installation method..."
select={false}
size={null}
values={[{"group":"Package managers","label":"Nix","value":"nix"},{"group":"Nones","label":"Vector CLI","value":"vector-cli"},{"group":"Platforms","label":"Docker CLI","value":"docker-cli"},{"group":"Platforms","label":"Docker Compose","value":"docker-compose"}]}>
<TabItem value="nix">

<Steps headingDepth={3}>

1.  ### Install Vector

    ```bash
    nix-env --file https://github.com/NixOS/nixpkgs/archive/master.tar.gz --install --attr vector
    ```

    <CodeExplanation>

    - The `--file` flag ensures that you're installing the latest stable version
      of Vector (0.10.0).
    - The `--attr` improves installation speed.

    </CodeExplanation>

    [Looking for a specific version?][docs.package_managers.nix#versions]

2.  ### Configure Vector

    <ConfigExample
    format="toml"
    path={"/etc/vector/vector.toml"}
    sourceName={"journald"}
    sinkName={null} />

3.  ### Start Vector

    ```bash
    vector --config /etc/vector/vector.toml
    ```

    <CodeExplanation>

    - `vector` is placed in your `$PATH`.
    - You must create a [Vector configuration file][docs.configuration] to
      successfully start Vector.

    </CodeExplanation>

</Steps>

</TabItem>
<TabItem value="vector-cli">

<Steps headingDepth={3}>
<ol>
<li>

### Install Vector

<InstallationCommand />

Or choose your [preferred method][docs.installation].

</li>
<li>

### Configure Vector

<ConfigExample
format="toml"
path={"vector.toml"}
sourceName={"journald"}
sinkName={null} />

</li>
<li>

### Start Vector

```bash
vector --config vector.toml
```

That's it! Simple and to the point. Hit `ctrl+c` to exit.

</li>
</ol>
</Steps>

</TabItem>
<TabItem value="docker-cli">

<Steps headingDepth={3}>
<ol>
<li>

### Configure Vector

<ConfigExample
format="toml"
path={"/etc/vector/vector.toml"}
sourceName={"journald"}
sinkName={null} />

</li>
<li>

### Start the Vector container

```bash
docker run \
  -v $PWD/vector.toml:/etc/vector/vector.toml:ro \
  timberio/vector:latest-alpine
```

<CodeExplanation>

- The `-v $PWD/vector.to...` flag passes your custom configuration to Vector.
- The `timberio/vector:latest-alpine` is the default image we've chosen, you are welcome to use [other image variants][docs.platforms.docker#variants].

</CodeExplanation>

That's it! Simple and to the point. Hit `ctrl+c` to exit.

</li>
</ol>
</Steps>

</TabItem>
<TabItem value="docker-compose">

compose!

</TabItem>
</Tabs>
</TabItem>
</Tabs>

[docs.configuration]: /docs/setup/configuration/
[docs.installation]: /docs/setup/installation/
[docs.package_managers.nix#versions]: /docs/setup/installation/package-managers/nix/#versions
[docs.platforms.docker#variants]: /docs/setup/installation/platforms/docker/#variants
[docs.sources.journald]: /docs/reference/sources/journald/
[docs.sources]: /docs/reference/sources/
[docs.strategies#daemon]: /docs/setup/deployment/strategies/#daemon
