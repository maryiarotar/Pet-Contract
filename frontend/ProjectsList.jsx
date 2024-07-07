import React, { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';

const ProjectsList = () => {
  const [projects, setProjects] = useState([]);

  useEffect(() => {
    const fetchProjects = async () => {
      try {
        const response = await fetch('http://127.0.0.1:3000/projects');
        const data = await response.json();
        setProjects(data);
      } catch (error) {
        console.error('Failed to fetch projects:', error);
      }
    };

    fetchProjects();
  }, []);

  return (
    <div style={{ background: 'linear-gradient(to right, #FF4500, gray)'}}>
      <header style={{ background: 'linear-gradient(to right, #808080, #000, #808080)', fontFamily: 'Roboto, sans-serif', width: '100%', textAlign: 'center', padding: '20px 0', marginBottom: '20px', boxShadow: '0px 5px 5px rgba(0, 0, 0, 0.2)', width: '100%', textAlign: 'center', padding: '20px 0', marginBottom: '20px' }}>
        <h2 style={{ color: 'white' }}>PROJECTS</h2>
      </header>
      <div style={{ display: 'flex', justifyContent: 'center', alignItems: 'center' }}>
      <div style={{ width: '75%', padding: 25 }}>
        <ul style={{ listStyleType: 'none', padding: 0, display: 'flex', flexWrap: 'wrap' }}>
          {projects.map((project, index) => (
            <>
             <li key={index} style={{ width: '33%', marginBottom: '20px', borderBottom: '1px solid #ccc', paddingBottom: '20px' }}>
              <h3 style={{ height: '70px', overflow: 'hidden' }}>{project.name}</h3>
              <p><strong>Тема:</strong> {project.topic}</p>
              <a href={`/projects/${project.id}`} style={{ textDecoration: 'none', color: 'inherit' }}>
              <p><strong></strong> <img src={project.file_path} alt={project.name} style={{ width: '200px', height: '200px', objectFit: 'cover' }} /></p>
              <p><strong>Описание:</strong> {project.description.slice(0, 55)}...</p></a>
              <p><strong>Автор:</strong> {project.author}</p>
              <p><strong>Дата публикации:</strong> {project.date_published}</p>
            </li></>
          ))}
        </ul>
      </div>
    </div>
    <footer style={{ background: 'linear-gradient(to right, #000, #808080, #000)', color: 'white', width: '100%', height: '8%', textAlign: 'center', padding: '10px', position: 'fixed', bottom: '0' }}>
            <Link to="/donation" className="btn-orange-red" style={{marginRight: '50px'}}><b>To Make Donation</b></Link>
            <Link to="/save-project" className="btn-orange-red"><b>Propose a project</b></Link>
      </footer>
    </div>
  );
};

export default ProjectsList;
