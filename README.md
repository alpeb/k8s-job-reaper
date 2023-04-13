# k8s-job-reaper

Keeping Kubernetes Clean and Mean!

## Preliminary Context

I'd like to have a Kubernetes Job spawn Pods that perform some work in parallel,
and remove them all once the first Pod finishes.

The [Jobs](https://kubernetes.io/docs/concepts/workloads/controllers/job/) docs
aren't very clear about this particular scenario.

My first intuition was to set the `parallelism` field to the number of
concurrent Pods, and `completions` to 1, because according to the docs that
field

> Specifies the desired number of successfully finished pods the job should be run with

but shortly after it states:

> Setting to 1 means that parallelism is limited to 1

so no cigar.

After some trial and error I settled with leaving `completions` unset. That
indeed marks the Job as completed when the first Pod finishes, but it leaves the
other Pods running, which in my particular case resulted in wasted resources.

## The Problem

The problem this operator solves is to delete a Job and its children Pods as
soon as the Job is marked Completed, even if there still are Pods in a Running
state.

## Cluster Install

```bash
helm install job-reaper -n <job-namespace> chart/job-reaper
```

## Local Usage

```
Usage: k8s-job-reaper [OPTIONS] --namespace <NAMESPACE>

Options:
      --namespace <NAMESPACE>
      --log-level <LOG_LEVEL>  [env: LOG_LEVEL=] [default: info]
  -h, --help                   Print help
  -V, --version                Print version
```
