# mps

microbe platform system

| crate                 | description                              | status        |
| -----------------     | ---------------------------------------- | ------------- |
| [mps_cli][0]          | mps cli                                  | in progress   |
| [mps_cloud][6]        | responsible for cloud manager            | in progress   |
| [mps_config][3]       | responsible for reading config files     | in progress   |
| [mps_container][4]    | responsible for containers manager       | in progress   |
| [mps_im][2]           | responsible instante messaging manager   | in progress   |
| [mps_log][7]           | log | in progress   |
| [mps_orchestrator][5] | responsible orchestrator manager         | in progress   |
| [mps_render][8]       | responsible render templates of projects | in progress   |
| [mps_scm][1]          | responsible source control manager       | in progress   |
| mps_project          | | |
| mps_group | | |
| mps_monitoring | | |

## create "project"

1. Create github repo
```
in: project_name
out: project_name, git_url
```

2. Clone sample (clone boilerplate template)
```
in: nome_repo,
out: folder clone
```

3. Render template (generate boilerplate)
```
in: projetc_name
out: out_path (folder boilerplate)
```

4. Create registry repo (aws ecr)
```
in: projetc_name, tag
out: endpoint
```

5. Docker build, auth registry, push image
```
in: boiler_plate path, dockerfile path, tag
out: endpoint
```

6. Orchestrator (create artifcats and post do k8s,)
```
in: projetc_name
out: -
```

7. Load balancers url

[0]: ./crates/mps_cli/README.md
[1]: ./crates/mps_scm/README.md
[2]: ./crates/mps_im/README.md
[3]: ./crates/mps_config/README.md
[4]: ./crates/mps_container/README.md
[5]: ./crates/mps_orchestrator/README.md
[6]: ./crates/mps_cloud/README.md
[7]: ./crates/mps_render/README.md
[8]: ./crates/mps_log/README.md
