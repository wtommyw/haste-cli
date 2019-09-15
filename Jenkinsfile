import groovy.json.JsonSlurperClassic

podTemplate(label: 'jenkins-pipeline', containers: [
    containerTemplate(name: 'jnlp', image: 'lachlanevenson/jnlp-slave:3.10-1-alpine', args: '${computer.jnlpmac} ${computer.name}', workingDir: '/home/jenkins', resourceRequestCpu: '200m', resourceLimitCpu: '300m', resourceRequestMemory: '256Mi', resourceLimitMemory: '512Mi'),
    containerTemplate(name: 'rust', image: 'rust-musl-builder:latest', command: 'cat', ttyEnabled: true),
]){

  node ('jenkins-pipeline') {

    checkout scm

    stage('test') {
      container('rust') {
        sh "cargo test"
      }
    }
    
  }
}
