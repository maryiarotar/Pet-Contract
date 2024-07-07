import React, { useEffect, useState } from 'react';
import { useParams } from 'react-router-dom';
import { Link } from 'react-router-dom';

const ProjectDetails = () => {
  const { projectId } = useParams();
  const [projectDetails, setProjectDetails] = useState(null);

  useEffect(() => {
    const fetchProjectDetails = async () => {
      try {
        const response = await fetch('http://127.0.0.1:3000/projects');
        const projects = await response.json();
        // Find the project with matching projectId
        const project = projects.find(project => project.id === Number(projectId));
        if (project) {
          // Save name and description of the project to state
          setProjectDetails({ 
            name: project.name, 
            description: project.description,
            file_path: project.file_path,
            date_published: project.date_published,
            author: project.author
        });
        } else {
          console.log(`Project with ID ${projectId} not found.`);
        }
      } catch (error) {
        console.error('Failed to fetch project details:', error);
      }
    };

    fetchProjectDetails();
  }, [projectId]);

  if (!projectDetails) {
    return <div>Loading...</div>;
  }

  return (
    <div style={{ background: 'linear-gradient(to right, #000000, gray)', paddingBottom: '150px', paddingTop: '50px', paddingLeft: '400px' }}>
        <h2 style={{ color: 'white' }}>Project Details</h2>
        <div style={{ border: '1px solid #ccc', padding: '20px', borderRadius: '10px', background: 'gray', maxWidth: '800px', align: 'center', alignItems: 'center', }}>
              <p><strong>Name:</strong> {projectDetails.name}</p>
              <div ><p><strong>Описание:</strong> {projectDetails.description}</p></div>
              <img src={projectDetails.file_path} alt={projectDetails.name} style={{ width: '200px', height: '200px', objectFit: 'cover' }} />
              <p><strong>Автор:</strong> <b>{projectDetails.author}</b></p>
              <p><strong>Дата публикации:</strong> {projectDetails.date_published}</p>

        <footer style={{background: 'linear-gradient(to right, #000, #808080, #000)', color: 'white',  width: '100%', height: '20%', padding: '10px', display: 'flex', justifyContent: 'center', alignItems: 'center'}}>
        <a href={`/donation/${projectDetails.author}/${projectDetails.name}`}
            className="btn-orange-red" > <b>To Make Donation</b> </a>
        </footer>
        </div>
    </div>
  );
};

export default ProjectDetails;

/*import { useParams } from 'react-router-dom';

const ProjectDetails = ({ projects }) => {
  const { projectId } = useParams();

  // Найти проект по его id
  const project = projects.find(project => project.id === parseInt(projectId));

  if (!project) {
    return <div>Проект не найден</div>;
  }

  return (
    <div>
      <h2>{project.name}</h2>
      <p><strong>Тема:</strong> {project.topic}</p>
      <p><strong>Описание:</strong> {project.description}</p>
      <p><strong>Автор:</strong> {project.author}</p>
      <p><strong>Дата публикации:</strong> {project.date_published}</p>
      <p><strong>Путь к фото:</strong> <img src={project.file_path} alt={project.name} style={{ width: '200px', height: '200px', objectFit: 'cover' }} /></p>
    </div>
  );
};

export default ProjectDetails;
*/