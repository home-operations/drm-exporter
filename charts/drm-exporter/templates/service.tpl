apiVersion: v1
kind: Service
metadata:
  name: {{ include "drm-exporter.fullname" . }}
  labels:
    {{- include "drm-exporter.labels" . | nindent 4 }}
  {{- with .Values.service.annotations }}
  annotations:
    {{- tpl (toYaml .) $ | nindent 4 }}
  {{- end }}
spec:
  type: {{ .Values.service.type }}
  selector:
    {{- include "drm-exporter.selectorLabels" . | nindent 4 }}
  ports:
    - name: metrics
      port: {{ .Values.config.port }}
      targetPort: metrics
      protocol: TCP
