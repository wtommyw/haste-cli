import groovy.json.JsonSlurperClassic

podTemplate(label: 'jenkins-pipeline', containers: [
    containerTemplate(name: 'jnlp', image: 'lachlanevenson/jnlp-slave:3.10-1-alpine', args: '${computer.jnlpmac} ${computer.name}', workingDir: '/home/jenkins/agent', resourceRequestCpu: '200m', resourceLimitCpu: '300m', resourceRequestMemory: '256Mi', resourceLimitMemory: '512Mi'),
    containerTemplate(name: 'rust', image: 'ekidd/rust-musl-builder:latest', workingDir: '/home/jenkins/agent', command: 'cat', ttyEnabled: true),
]){

  node ('jenkins-pipeline') {

    checkout scm

    stage('test') {
      sh 'whoami'
    }

  }
}
