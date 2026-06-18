{{- if .Values.dra.enabled }}
apiVersion: {{ .Values.dra.apiVersion }}
kind: ResourceClaimTemplate
metadata:
  name: {{ include "drm-exporter.resourceClaimTemplateName" . }}
  labels:
    {{- include "drm-exporter.labels" . | nindent 4 }}
  {{- with .Values.dra.annotations }}
  annotations:
    {{- tpl (toYaml .) $ | nindent 4 }}
  {{- end }}
spec:
  # ResourceClaimTemplate.spec.spec is the ResourceClaimSpec stamped into every
  # generated claim. The default is an admin-access monitor request: read-only
  # visibility of all GPUs on the node, allocated as `All` and not counted as
  # consumed (so it never steals a VF from a workload).
  spec:
    devices:
      requests:
        - name: {{ .Values.dra.requestName | default (include "drm-exporter.fullname" .) }}
          exactly:
            deviceClassName: {{ required "dra.deviceClassName is required when dra.enabled (e.g. gpu.intel.com or gpu.nvidia.com)" .Values.dra.deviceClassName }}
            {{- if .Values.dra.adminAccess }}
            adminAccess: true
            {{- end }}
            allocationMode: {{ .Values.dra.allocationMode }}
            {{- if eq .Values.dra.allocationMode "ExactCount" }}
            count: {{ .Values.dra.count }}
            {{- end }}
            {{- with .Values.dra.tolerations }}
            tolerations:
              {{- toYaml . | nindent 14 }}
            {{- end }}
{{- end }}
