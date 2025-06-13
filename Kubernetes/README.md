# Kubernetes

### Creating a YAML file from an existing image for own projects.

```
kubectl run nginx --image=nginx --dry-run=client -o yaml
```

### Ways of applying the created yaml file to the cluster

1. kubectl create -f nginx.yaml
2. kubectl apply -f nginx.yaml

#### The difference between the two...

## Creating Deployments

### Getting help of deployments

```
kubectl create deployment -h | less
```

Example of creating a deployment from `nginx` image with 3 replicas

```
kubectl create deploy test-depl --image=nginx --replicas=3
```
