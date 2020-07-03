node {
    checkout scm
    docker.withRegistry('https://docker.pkg.github.com', 'github') {
        stage('Build Server') {
            docker.build("docker.pkg.github.com/rustic-music-player/library-sync/server:latest", './packages/server').push()
        }
    }
}
